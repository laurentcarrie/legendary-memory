use std::path::{Path, PathBuf};
use yamake::model::GNode;

pub struct LyTexFile {
    pub path: PathBuf,
}

impl LyTexFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for LyTexFile {
    fn tag(&self) -> String {
        "lytex".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Find predecessors of type "lilypond"
        let ly_predecessors: Vec<_> = predecessors
            .iter()
            .filter(|p| p.tag() == "lilypond")
            .collect();

        if ly_predecessors.len() != 1 {
            log::error!(
                "LyTexFile {} expects exactly one lilypond predecessor, found {}",
                self.path.display(),
                ly_predecessors.len()
            );
            return false;
        }

        let ly_file = ly_predecessors[0];
        let ly_pathbuf = ly_file.pathbuf();
        let ly_filename = ly_pathbuf
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.ly");

        let full_path = sandbox.join(&self.path);

        // Create parent directory if needed
        if let Some(parent) = full_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!(
                    "Failed to create directory for {}: {}",
                    self.path.display(),
                    e
                );
                return false;
            }
        }

        let content = format!("\\lilypondfile{{{ly_filename}}}\n");

        if let Err(e) = std::fs::write(&full_path, &content) {
            log::error!("Failed to write {}: {}", self.path.display(), e);
            return false;
        }

        true
    }
}
