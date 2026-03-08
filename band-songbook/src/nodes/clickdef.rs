use std::path::{Path, PathBuf};
use yamake::model::GRootNode;

use crate::model::ClicksDefinition;

/// Root node for a clicks definition file that validates it at construction time.
pub struct ClickDef {
    pub path: PathBuf,
    pub data: ClicksDefinition,
}

impl ClickDef {
    /// Creates a new `ClickDef` node by reading and validating the YAML file.
    ///
    /// `path` is the relative path (e.g. `"artist/song/clicks-def.yml"`).
    /// `srcdir` is the root directory where source files live.
    pub fn new(path: PathBuf, srcdir: &Path) -> Result<Self, String> {
        let full_path = srcdir.join(&path);
        let yaml = std::fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read {}: {e}", full_path.display()))?;
        let data: ClicksDefinition = serde_yaml::from_str(&yaml)
            .map_err(|e| format!("Invalid clicks definition {}: {e}", full_path.display()))?;
        Ok(Self { path, data })
    }
}

impl GRootNode for ClickDef {
    fn tag(&self) -> String {
        "clicks-def".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_def_load_valid() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- beat_number: 1\n  time: 0.0\n  description: bar 1\n- beat_number: 5\n  time: 1.714\n  description: bar 2\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_ok(), "should parse valid clicks definition");

        let node = node.unwrap();
        assert_eq!(node.data.clicks.len(), 2);
        assert_eq!(node.data.clicks[0].beat_number, 1);
        assert!((node.data.clicks[0].time - 0.0).abs() < 1e-9);
        assert_eq!(node.data.clicks[0].description, "bar 1");
        assert_eq!(node.data.clicks[1].beat_number, 5);
        assert_eq!(node.tag(), "clicks-def");
    }

    #[test]
    fn test_click_def_load_invalid() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(&def_path, "this is not valid").unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_err(), "should reject invalid clicks definition");
    }

    #[test]
    fn test_click_def_load_missing() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_err(), "should fail for missing file");
    }
}
