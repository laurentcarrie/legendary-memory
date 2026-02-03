use std::path::{Path, PathBuf};
use yamake::model::GNode;

pub struct SongTikz {
    pub path: PathBuf,
}

impl SongTikz {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for SongTikz {
    fn tag(&self) -> String {
        "songtikz".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, _sandbox: &Path, _predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        true
    }

    fn scan(
        &self,
        _sandbox: &Path,
        _predecessors: &[&(dyn GNode + Send + Sync)],
    ) -> (bool, Vec<PathBuf>) {
        (true, Vec::new())
    }
}
