use std::path::{Path, PathBuf};
use std::process::Command;
use yamake::command::run_command;
use yamake::model::GNode;

pub struct TexOfLilypond {
    pub path: PathBuf,
}

impl TexOfLilypond {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for TexOfLilypond {
    fn tag(&self) -> String {
        "texoflilypond".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Find predecessors of type "lytex"
        let lytex_predecessors: Vec<_> =
            predecessors.iter().filter(|p| p.tag() == "lytex").collect();

        if lytex_predecessors.len() != 1 {
            log::error!(
                "TexOfLilypond {} expects exactly one lytex predecessor, found {}",
                self.path.display(),
                lytex_predecessors.len()
            );
            return false;
        }

        let lytex_file = lytex_predecessors[0];
        let lytex_pathbuf = lytex_file.pathbuf();
        let lytex_filename = lytex_pathbuf
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.lytex");

        // self.path = parent/xxx.output/xxx.tex
        // output_dir = parent/xxx.output
        let output_dir = match self.path.parent() {
            Some(p) => p,
            None => {
                log::error!("TexOfLilypond path has no parent: {}", self.path.display());
                return false;
            }
        };

        let output_full_path = sandbox.join(output_dir);

        // Working directory is the parent of the output directory
        let workdir = output_full_path.parent().unwrap_or(sandbox);

        let mut cmd = Command::new("lilypond-book");
        cmd.arg("--output")
            .arg(output_dir.file_name().unwrap_or_default())
            .arg("--pdf")
            .arg("--latex-program=lualatex")
            .arg(lytex_filename)
            .current_dir(workdir);

        let node_id = self.path.to_string_lossy();
        let success = run_command(&mut cmd, sandbox, &node_id);
        if !success {
            log::error!(
                "lilypond-book failed for {} (lytex: {})",
                self.path.display(),
                lytex_filename
            );
        }
        success
    }
}
