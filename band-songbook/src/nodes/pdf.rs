use std::path::{Path, PathBuf};
use std::process::Command;
use yamake::model::GNode;

pub struct PdfFile {
    pub path: PathBuf,
}

impl PdfFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for PdfFile {
    fn tag(&self) -> String {
        "pdf".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Working directory is the directory containing the PDF file
        let workdir = sandbox.join(self.path.parent().unwrap_or(Path::new("")));

        // Find the tex predecessor with the same stem as the pdf (e.g., main.tex for main.pdf)
        let pdf_stem = self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let tex_file = match predecessors.iter().find(|p| {
            p.tag() == "tex"
                && p.pathbuf()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s == pdf_stem)
                    .unwrap_or(false)
        }) {
            Some(pred) => *pred,
            None => {
                log::error!(
                    "PdfFile {} has no matching tex predecessor",
                    self.path.display()
                );
                return false;
            }
        };

        // Tex file path relative to workdir (just the filename)
        let tex_pathbuf = tex_file.pathbuf();
        let tex_filename = tex_pathbuf.file_name().unwrap_or_default();
        let pdf_filename = self.path.file_name().unwrap_or_default();
        let pdf_path = workdir.join(pdf_filename);

        // Ensure workdir exists
        if let Err(e) = std::fs::create_dir_all(&workdir) {
            log::error!("Failed to create workdir: {e}");
            return false;
        }

        // Setup logging
        let node_id = self.path.to_string_lossy();
        let stdout_path = sandbox.join("logs").join(format!("{}.stdout", &node_id));
        let logs_dir = stdout_path.parent().unwrap_or(Path::new(""));
        let _ = std::fs::create_dir_all(logs_dir);

        log::info!(
            "Building {} in {}",
            pdf_filename.to_string_lossy(),
            workdir.display()
        );

        for run in 1..=3 {
            let output = Command::new("lualatex")
                .arg("-interaction=nonstopmode")
                .arg(tex_filename)
                .current_dir(&workdir)
                .output();

            let stdout_content = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    // Save stdout and stderr for debugging
                    let _ = std::fs::write(&stdout_path, &stdout);
                    let stderr_path = sandbox.join("logs").join(format!("{}.stderr", &node_id));
                    let _ = std::fs::write(&stderr_path, &stderr);
                    if !out.status.success() {
                        log::error!(
                            "lualatex failed with status {:?} for {}",
                            out.status.code(),
                            self.path.display()
                        );
                        log::error!("stdout : {stdout}");
                        log::error!("stderr : {stderr}");
                    }
                    stdout
                }
                Err(e) => {
                    log::error!("Failed to run lualatex: {} for {}", e, self.path.display());
                    let stderr_path = sandbox.join("logs").join(format!("{}.stderr", &node_id));
                    let _ = std::fs::write(&stderr_path, format!("Failed to run lualatex: {e}"));
                    return false;
                }
            };

            // Check if PDF was created (lualatex may exit with error but still create PDF)
            if !pdf_path.exists() {
                log::error!(
                    "lualatex did not create PDF after run {} ; {}",
                    run,
                    &self.pathbuf().display()
                );
                // Log the output for debugging
                // log::error!("LaTeX output: {}", stdout_content);
                return false;
            }

            // Log LaTeX errors but don't fail if PDF was created (nonstopmode continues)
            if stdout_content.contains("! LaTeX Error:") {
                log::warn!(
                    "LaTeX errors detected in {} (PDF still created)",
                    self.path.display()
                );
            }

            // Fail on undefined control sequence
            if stdout_content.contains("! Undefined control sequence.") {
                log::error!("Undefined control sequence in {}", self.path.display());
                return false;
            }

            // Check if we need to rerun for references
            let needs_rerun = stdout_content.contains("Rerun to get the references right");

            if !needs_rerun {
                log::info!("LaTeX completed after {run} run(s)");
                break;
            }

            if run < 3 {
                log::info!("LaTeX needs rerun for references (run {run}/3)");
            }
        }

        // Verify the PDF exists
        pdf_path.exists()
    }

    fn scan(
        &self,
        sandbox: &Path,
        predecessors: &[&(dyn GNode + Send + Sync)],
    ) -> (bool, Vec<PathBuf>) {
        let (_, inputs, _) = self.scan_with_toplevel_ly(sandbox, predecessors);
        (true, inputs)
    }
}

impl PdfFile {
    // TODO: when parsing and discovering dependencies to lilypond files,
    // check that it is coherent with the files declared in song.yml

    /// Scan for inputs, returning (success, all_inputs, toplevel_ly_files)
    /// toplevel_ly_files are .ly files directly referenced from tex via \lyfile{} or \songly{}
    pub fn scan_with_toplevel_ly(
        &self,
        sandbox: &Path,
        predecessors: &[&(dyn GNode + Send + Sync)],
    ) -> (bool, Vec<PathBuf>, Vec<PathBuf>) {
        use std::collections::HashSet;

        let mut inputs = Vec::new();
        let mut toplevel_ly = Vec::new();
        let mut visited: HashSet<PathBuf> = HashSet::new();

        // Start with predecessor paths
        let mut to_scan: Vec<PathBuf> = predecessors.iter().map(|p| p.pathbuf()).collect();

        while let Some(file_path) = to_scan.pop() {
            if visited.contains(&file_path) {
                continue;
            }
            visited.insert(file_path.clone());

            let full_path = sandbox.join(&file_path);
            let parent_dir = file_path.parent().unwrap_or(Path::new(""));

            let content = match std::fs::read_to_string(&full_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for line in content.lines() {
                // Skip LaTeX comments
                if line.trim_start().starts_with('%') {
                    continue;
                }
                // Detect \input{xxx} -> xxx.tex
                if let Some(start) = line.find("\\input{") {
                    let rest = &line[start + 7..];
                    if let Some(end) = rest.find('}') {
                        let input_file = &rest[..end];
                        let mut input_path = parent_dir.join(input_file);
                        // Add .tex extension if no extension present
                        if input_path.extension().is_none() {
                            input_path.set_extension("tex");
                        }
                        inputs.push(input_path.clone());
                        // Add to scan queue for recursive scanning
                        to_scan.push(input_path);
                    }
                }
                // Detect \lyfile{xxx} or \songly{xxx} -> xxx.ly (top-level ly files)
                for (pattern, len) in [("\\lyfile{", 8), ("\\songly{", 8)] {
                    if let Some(start) = line.find(pattern) {
                        let rest = &line[start + len..];
                        if let Some(end) = rest.find('}') {
                            let ly_file = &rest[..end];
                            let mut ly_path = parent_dir.join(ly_file);
                            // Add .ly extension if no extension present
                            if ly_path.extension().is_none() {
                                ly_path.set_extension("ly");
                            }
                            inputs.push(ly_path.clone());
                            toplevel_ly.push(ly_path.clone());
                            // Add to scan queue for recursive scanning of .ly files
                            to_scan.push(ly_path);
                        }
                    }
                }
                // Detect \include "filename" in lilypond files (included, not top-level)
                if let Some(start) = line.find("\\include") {
                    let rest = &line[start + 8..];
                    // Find opening quote
                    if let Some(quote_start) = rest.find('"') {
                        let after_quote = &rest[quote_start + 1..];
                        // Find closing quote
                        if let Some(quote_end) = after_quote.find('"') {
                            let include_file = &after_quote[..quote_end];
                            let include_path = parent_dir.join(include_file);
                            inputs.push(include_path.clone());
                            // Add to scan queue for recursive scanning
                            to_scan.push(include_path);
                        }
                    }
                }
            }
        }

        (true, inputs, toplevel_ly)
    }
}
