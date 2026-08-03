use std::path::{Path, PathBuf};
use yamake::model::GRootNode;

use crate::model::RawClicksDefinition;

/// Root node for a clicks definition file that validates it at construction time.
pub struct ClickDef {
    pub path: PathBuf,
    pub data: RawClicksDefinition,
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
        let data: RawClicksDefinition = serde_yaml::from_str(&yaml)
            .map_err(|e| format!("Invalid clicks definition {}: {e}", full_path.display()))?;
        // Validate: each entry has exactly one of bar_number or section_id
        for (i, click) in data.clicks.iter().enumerate() {
            match (&click.bar_number, &click.section_id) {
                (None, None) => {
                    return Err(format!(
                        "clicks-def entry {i}: must have either bar_number or section_id"
                    ));
                }
                (Some(_), Some(_)) => {
                    return Err(format!(
                        "clicks-def entry {i}: bar_number and section_id are mutually exclusive"
                    ));
                }
                _ => {}
            }
        }
        // Validate: times must be strictly increasing
        for i in 1..data.clicks.len() {
            let prev = &data.clicks[i - 1];
            let curr = &data.clicks[i];
            if curr.time <= prev.time {
                return Err(format!(
                    "clicks-def entry {i}: time {:.3} is not greater than previous time {:.3} — times must be strictly increasing",
                    curr.time, prev.time
                ));
            }
        }
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
    fn test_click_def_load_valid_bar_number() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- bar_number: 1\n  beat_in_bar_number: 1\n  time: \"0:0.0\"\n  description: bar 1\n- bar_number: 2\n  beat_in_bar_number: 1\n  time: \"0:1.714\"\n  description: bar 2\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_ok(), "should parse valid clicks definition");

        let node = node.unwrap();
        assert_eq!(node.data.clicks.len(), 2);
        assert_eq!(node.data.clicks[0].bar_number, Some(1));
        assert_eq!(node.data.clicks[0].beat_in_bar_number, 1);
        assert!((node.data.clicks[0].time - 0.0).abs() < 1e-9);
        assert_eq!(node.data.clicks[0].description, "bar 1");
        assert_eq!(node.data.clicks[1].bar_number, Some(2));
        assert_eq!(node.tag(), "clicks-def");
    }

    #[test]
    fn test_click_def_load_valid_section_id() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- section_id: intro\n  beat_in_bar_number: 1\n  time: \"0:0.0\"\n  description: intro\n- section_id: couplet1\n  beat_in_bar_number: 1\n  time: \"0:8.0\"\n  description: couplet 1\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_ok(), "should parse section_id clicks definition");

        let node = node.unwrap();
        assert_eq!(node.data.clicks.len(), 2);
        assert_eq!(node.data.clicks[0].section_id, Some("intro".to_string()));
        assert_eq!(node.data.clicks[0].bar_number, None);
    }

    #[test]
    fn test_click_def_rejects_both_fields() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- bar_number: 1\n  section_id: intro\n  beat_in_bar_number: 1\n  time: \"0:0.0\"\n  description: both\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(
            node.is_err(),
            "should reject entry with both bar_number and section_id"
        );
    }

    #[test]
    fn test_click_def_rejects_neither_field() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- beat_in_bar_number: 1\n  time: \"0:0.0\"\n  description: missing\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(
            node.is_err(),
            "should reject entry with neither bar_number nor section_id"
        );
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

    #[test]
    fn test_click_def_rejects_non_monotonic_times() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- bar_number: 1\n  beat_in_bar_number: 1\n  time: \"0:5.0\"\n  description: bar 1\n- bar_number: 2\n  beat_in_bar_number: 1\n  time: \"0:3.0\"\n  description: bar 2\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_err(), "should reject non-monotonic times");
        assert!(node.err().unwrap().contains("not greater than previous"));
    }

    #[test]
    fn test_click_def_rejects_equal_times() {
        let srcdir = tempfile::tempdir().expect("Failed to create temp dir");
        let def_path = srcdir.path().join("clicks-def.yml");
        std::fs::write(
            &def_path,
            "clicks:\n- bar_number: 1\n  beat_in_bar_number: 1\n  time: \"0:5.0\"\n  description: bar 1\n- bar_number: 2\n  beat_in_bar_number: 1\n  time: \"0:5.0\"\n  description: bar 2\n",
        )
        .unwrap();

        let node = ClickDef::new(PathBuf::from("clicks-def.yml"), srcdir.path());
        assert!(node.is_err(), "should reject equal times");
    }
}
