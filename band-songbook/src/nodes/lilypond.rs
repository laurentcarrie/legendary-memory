use std::path::PathBuf;
use yamake::model::GRootNode;

pub struct LilypondFile {
    pub path: PathBuf,
}

impl LilypondFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GRootNode for LilypondFile {
    fn tag(&self) -> String {
        "lilypond".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
}
