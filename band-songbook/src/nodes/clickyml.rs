use std::path::{Path, PathBuf};
use yamake::model::GRootNode;

use crate::model::ClicksDefinition;

/// Root node for `clicks.yml` that validates the file at construction time.
pub struct ClickYml {
    pub path: PathBuf,
    pub data: ClicksDefinition,
}

impl ClickYml {
    /// Creates a new `ClickYml` node by reading and validating the YAML file.
    ///
    /// `path` is the relative path (e.g. `"artist/song/clicks.yml"`).
    /// `srcdir` is the root directory where source files live.
    pub fn new(path: PathBuf, srcdir: &Path) -> Result<Self, String> {
        let full_path = srcdir.join(&path);
        let yaml = std::fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read {}: {e}", full_path.display()))?;
        let data: ClicksDefinition = serde_yaml::from_str(&yaml)
            .map_err(|e| format!("Invalid clicks.yml {}: {e}", full_path.display()))?;
        Ok(Self { path, data })
    }
}

impl GRootNode for ClickYml {
    fn tag(&self) -> String {
        "clicks.yml".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_yml_load_valid() {
        // Write a valid clicks.yml to a temp directory
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let clicks_path = srcdir.path().join("clicks.yml");
        std::fs::write(&clicks_path, "ticks:\n- 0.0\n- 0.5\n- 1.0\n").unwrap();

        let node = ClickYml::new(PathBuf::from("clicks.yml"), srcdir.path());
        assert!(node.is_ok(), "should parse valid clicks.yml");

        let node = node.unwrap();
        assert_eq!(node.data.ticks.len(), 3);
        assert_eq!(node.data.ticks[0], 0.0);
        assert_eq!(node.data.ticks[1], 0.5);
        assert_eq!(node.data.ticks[2], 1.0);
        assert_eq!(node.tag(), "clicks.yml");
    }

    #[test]
    fn test_click_yml_load_invalid() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let clicks_path = srcdir.path().join("clicks.yml");
        std::fs::write(&clicks_path, "this is not valid").unwrap();

        let node = ClickYml::new(PathBuf::from("clicks.yml"), srcdir.path());
        assert!(node.is_err(), "should reject invalid clicks.yml");
    }

    #[test]
    fn test_click_yml_load_missing() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");

        let node = ClickYml::new(PathBuf::from("clicks.yml"), srcdir.path());
        assert!(node.is_err(), "should fail for missing file");
    }
}
