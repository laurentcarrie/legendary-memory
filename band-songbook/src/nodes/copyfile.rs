use std::path::{Path, PathBuf};
use yamake::model::GNode;

/// Build node that copies a file to the delivery directory.
pub struct CopyFile {
    pub path: PathBuf,
    pub predecessor_tag: String,
}

impl CopyFile {
    pub fn new(path: PathBuf, predecessor_tag: String) -> Self {
        Self {
            path,
            predecessor_tag,
        }
    }
}

impl GNode for CopyFile {
    fn tag(&self) -> String {
        self.predecessor_tag.clone()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        if predecessors.len() != 1 {
            log::error!(
                "CopyFile {} expected 1 predecessor, got {}",
                self.path.display(),
                predecessors.len()
            );
            return false;
        }

        let predecessor = predecessors[0];
        if predecessor.tag() != self.predecessor_tag {
            log::error!(
                "CopyFile {} expected predecessor tag '{}', got '{}'",
                self.path.display(),
                self.predecessor_tag,
                predecessor.tag()
            );
            return false;
        }

        let src_path = sandbox.join(predecessor.pathbuf());
        let dest_path = sandbox.join(&self.path);

        // Create parent directory if needed
        if let Some(parent) = dest_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!(
                    "Failed to create directory for {}: {}",
                    self.path.display(),
                    e
                );
                return false;
            }
        }

        if let Err(e) = std::fs::copy(&src_path, &dest_path) {
            log::error!(
                "Failed to copy from {} to {}: {}",
                src_path.display(),
                dest_path.display(),
                e
            );
            return false;
        }

        log::info!("Copied {} to {}", src_path.display(), dest_path.display());

        true
    }
}
