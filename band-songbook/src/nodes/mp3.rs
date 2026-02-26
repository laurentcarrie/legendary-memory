use std::path::PathBuf;
use yamake::model::GRootNode;

/// Root node representing an MP3 audio file (`.mp3`).
pub struct Mp3 {
    pub path: PathBuf,
}

impl Mp3 {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GRootNode for Mp3 {
    fn tag(&self) -> String {
        "mp3".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
}
