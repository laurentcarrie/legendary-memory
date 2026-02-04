use std::path::{Path, PathBuf};
use yamake::model::GNode;

pub struct PdfCopyFile {
    pub path: PathBuf,
}

impl PdfCopyFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for PdfCopyFile {
    fn tag(&self) -> String {
        "pdfcopy".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Find the PDF predecessor
        let pdf_predecessor = match predecessors.iter().find(|p| p.tag() == "pdf") {
            Some(pred) => *pred,
            None => {
                log::error!("PdfCopyFile {} has no pdf predecessor", self.path.display());
                return false;
            }
        };

        let src_path = sandbox.join(pdf_predecessor.pathbuf());
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

        // Copy the PDF file
        if let Err(e) = std::fs::copy(&src_path, &dest_path) {
            log::error!(
                "Failed to copy PDF from {} to {}: {}",
                src_path.display(),
                dest_path.display(),
                e
            );
            return false;
        }

        log::info!(
            "Copied PDF from {} to {}",
            src_path.display(),
            dest_path.display()
        );

        true
    }
}
