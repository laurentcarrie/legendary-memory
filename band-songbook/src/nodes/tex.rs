use std::path::PathBuf;
use yamake::model::GRootNode;

pub struct TexFile {
    pub path: PathBuf,
}

impl TexFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GRootNode for TexFile {
    fn tag(&self) -> String {
        "tex".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
}
