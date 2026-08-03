use std::path::{Path, PathBuf};
use std::process::Command;
use yamake::model::GNode;

/// Build node for a standalone PDF of a single LilyPond file.
///
/// `lilypond-book` only emits per-snippet fragments inside content-hashed
/// subdirectories of `<stem>.output`, so this node runs `lilypond` directly with
/// `-dcrop` to get one tightly cropped page per snippet.
///
/// The tag is `pdfsnippet`, not `pdf`, so this is never mistaken for the song's
/// own `main.pdf` node.
pub struct PdfOfLilypond {
    pub path: PathBuf,
}

impl PdfOfLilypond {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for PdfOfLilypond {
    fn tag(&self) -> String {
        "pdfsnippet".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        let ly_predecessors: Vec<_> = predecessors
            .iter()
            .filter(|p| p.tag() == "lilypond")
            .collect();

        if ly_predecessors.len() != 1 {
            log::error!(
                "PdfOfLilypond {} expects exactly one lilypond predecessor, found {}",
                self.path.display(),
                ly_predecessors.len()
            );
            return false;
        }

        let ly_pathbuf = ly_predecessors[0].pathbuf();
        let ly_filename = match ly_pathbuf.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => {
                log::error!("PdfOfLilypond predecessor has no file name");
                return false;
            }
        };

        let stem = match self.path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => {
                log::error!(
                    "PdfOfLilypond path has no file stem: {}",
                    self.path.display()
                );
                return false;
            }
        };

        // lilypond runs in the song directory, which is where
        // `\include "macros.ly"` resolves from.
        let workdir = sandbox.join(self.path.parent().unwrap_or(Path::new("")));

        // Write under a scratch prefix: a bare `-o <stem>` would also emit
        // <stem>.midi and race MidiOfLilypond, which targets the same file.
        let scratch = format!("{stem}.snippet");

        log::info!(
            "Rendering snippet PDF {} from {ly_filename}",
            self.path.display()
        );

        let mut cmd = Command::new("lilypond");
        cmd.arg("-dcrop")
            .arg("-o")
            .arg(&scratch)
            .arg(ly_filename)
            .current_dir(&workdir);

        let node_id = self.path.to_string_lossy();
        let stdout_path = sandbox.join("logs").join(format!("{}.stdout", &node_id));
        let stderr_path = sandbox.join("logs").join(format!("{}.stderr", &node_id));
        let _ = std::fs::create_dir_all(stdout_path.parent().unwrap_or(Path::new("")));

        // Everything lilypond drops next to the cropped PDF, cleaned up after.
        let cropped = workdir.join(format!("{scratch}.cropped.pdf"));
        let leftovers = [
            workdir.join(format!("{scratch}.pdf")),
            workdir.join(format!("{scratch}.cropped.png")),
            workdir.join(format!("{scratch}.midi")),
        ];
        let sweep = || {
            for f in &leftovers {
                let _ = std::fs::remove_file(f);
            }
        };

        match cmd.output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let _ = std::fs::write(&stdout_path, &stdout);
                let _ = std::fs::write(&stderr_path, &stderr);

                if !out.status.success() {
                    log::error!(
                        "lilypond failed for {} ({ly_filename})",
                        self.path.display()
                    );
                    if !stderr.is_empty() {
                        log::error!("lilypond stderr: {stderr}");
                    }
                    sweep();
                    let _ = std::fs::remove_file(&cropped);
                    return false;
                }

                if !cropped.is_file() {
                    log::error!("lilypond produced no cropped PDF for {ly_filename}");
                    sweep();
                    return false;
                }

                if let Err(e) = std::fs::rename(&cropped, sandbox.join(&self.path)) {
                    log::error!("Failed to move {} into place: {e}", cropped.display());
                    sweep();
                    let _ = std::fs::remove_file(&cropped);
                    return false;
                }

                sweep();
                true
            }
            Err(e) => {
                log::error!("Failed to run lilypond: {e} for {}", self.path.display());
                let _ = std::fs::write(&stderr_path, format!("Failed to run lilypond: {e}"));
                sweep();
                false
            }
        }
    }
}
