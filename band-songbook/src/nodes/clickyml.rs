use std::path::{Path, PathBuf};
use yamake::model::GNode;

use crate::model::{Clicks, ClicksDefinition};

/// Build node for `clicks.yml` that generates click times by linear interpolation
/// from a `ClickDef` predecessor.
pub struct ClickYml {
    pub path: PathBuf,
}

impl ClickYml {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Interpolate click times from a `ClicksDefinition`.
///
/// For each pair of consecutive Click entries, linearly interpolates
/// the beat times between them. For example, if beat 1 is at 0.0s and
/// beat 5 is at 2.0s, beats 1–4 will be at 0.0, 0.5, 1.0, 1.5.
fn interpolate(def: &ClicksDefinition) -> Vec<f64> {
    if def.clicks.is_empty() {
        return vec![];
    }
    if def.clicks.len() == 1 {
        return vec![def.clicks[0].time];
    }

    let mut result = Vec::new();
    for window in def.clicks.windows(2) {
        let from = &window[0];
        let to = &window[1];
        let n_beats = to.beat_number - from.beat_number;
        for i in 0..n_beats {
            let t = from.time + (to.time - from.time) * (i as f64) / (n_beats as f64);
            result.push(t);
        }
    }
    // Add the last click's time
    result.push(def.clicks.last().unwrap().time);
    result
}

impl GNode for ClickYml {
    fn tag(&self) -> String {
        "clicks.yml".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Find the ClickDef predecessor
        let click_defs: Vec<_> = predecessors
            .iter()
            .filter(|p| p.tag() == "clicks-def")
            .collect();

        if click_defs.len() != 1 {
            log::error!(
                "ClickYml {} expected 1 clicks-def predecessor, got {}",
                self.path.display(),
                click_defs.len()
            );
            return false;
        }

        // Read the ClicksDefinition from the predecessor
        let def_path = sandbox.join(click_defs[0].pathbuf());
        let yaml = match std::fs::read_to_string(&def_path) {
            Ok(y) => y,
            Err(e) => {
                log::error!("Failed to read {}: {e}", def_path.display());
                return false;
            }
        };
        let def: ClicksDefinition = match serde_yaml::from_str(&yaml) {
            Ok(d) => d,
            Err(e) => {
                log::error!("Invalid clicks definition {}: {e}", def_path.display());
                return false;
            }
        };

        // Interpolate to generate all click times
        let clicks = Clicks {
            clicks: interpolate(&def),
        };

        // Write clicks.yml
        let out_path = sandbox.join(&self.path);
        if let Some(parent) = out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let yaml_out = match serde_yaml::to_string(&clicks) {
            Ok(y) => y,
            Err(e) => {
                log::error!("Failed to serialize clicks: {e}");
                return false;
            }
        };
        if let Err(e) = std::fs::write(&out_path, &yaml_out) {
            log::error!("Failed to write {}: {e}", out_path.display());
            return false;
        }

        log::info!(
            "Generated {} clicks in {}",
            clicks.clicks.len(),
            out_path.display()
        );
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Click;
    use crate::nodes::ClickDef;
    #[test]
    fn test_interpolate_two_points() {
        let def = ClicksDefinition {
            clicks: vec![
                Click {
                    beat_number: 1,
                    time: 0.0,
                    description: "bar 1".to_string(),
                },
                Click {
                    beat_number: 5,
                    time: 2.0,
                    description: "bar 2".to_string(),
                },
            ],
        };
        let result = interpolate(&def);
        assert_eq!(result.len(), 5); // beats 1,2,3,4,5
        assert!((result[0] - 0.0).abs() < 1e-9);
        assert!((result[1] - 0.5).abs() < 1e-9);
        assert!((result[2] - 1.0).abs() < 1e-9);
        assert!((result[3] - 1.5).abs() < 1e-9);
        assert!((result[4] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_interpolate_three_points() {
        let def = ClicksDefinition {
            clicks: vec![
                Click {
                    beat_number: 1,
                    time: 0.0,
                    description: "bar 1".to_string(),
                },
                Click {
                    beat_number: 3,
                    time: 1.0,
                    description: "bar 1 beat 3".to_string(),
                },
                Click {
                    beat_number: 5,
                    time: 3.0,
                    description: "bar 2".to_string(),
                },
            ],
        };
        let result = interpolate(&def);
        assert_eq!(result.len(), 5); // beats 1,2,3,4,5
        assert!((result[0] - 0.0).abs() < 1e-9);
        assert!((result[1] - 0.5).abs() < 1e-9);
        assert!((result[2] - 1.0).abs() < 1e-9);
        assert!((result[3] - 2.0).abs() < 1e-9);
        assert!((result[4] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_interpolate_empty() {
        let def = ClicksDefinition { clicks: vec![] };
        assert!(interpolate(&def).is_empty());
    }

    #[test]
    fn test_interpolate_single() {
        let def = ClicksDefinition {
            clicks: vec![Click {
                beat_number: 1,
                time: 0.0,
                description: "start".to_string(),
            }],
        };
        let result = interpolate(&def);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_click_yml_build() {
        let sandbox = tempfile::tempdir().expect("Failed to create sandbox");
        let song_dir = sandbox.path().join("artist/song");
        std::fs::create_dir_all(&song_dir).unwrap();

        // Write a clicks-def.yml
        std::fs::write(
            song_dir.join("clicks-def.yml"),
            "clicks:\n- beat_number: 1\n  time: 0.0\n  description: bar 1\n- beat_number: 5\n  time: 2.0\n  description: bar 2\n",
        )
        .unwrap();

        // Create ClickDef as predecessor
        let def_node =
            ClickDef::new(PathBuf::from("artist/song/clicks-def.yml"), sandbox.path()).unwrap();

        let click_yml_node = ClickYml::new(PathBuf::from("artist/song/clicks.yml"));

        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![&def_node];
        let result = click_yml_node.build(sandbox.path(), &predecessors);
        assert!(result, "build should succeed");

        // Verify the output
        let output =
            std::fs::read_to_string(sandbox.path().join("artist/song/clicks.yml")).unwrap();
        let clicks: Clicks = serde_yaml::from_str(&output).unwrap();
        assert_eq!(clicks.clicks.len(), 5);
        assert!((clicks.clicks[0] - 0.0).abs() < 1e-9);
        assert!((clicks.clicks[1] - 0.5).abs() < 1e-9);
        assert!((clicks.clicks[4] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_click_yml_missing_predecessor() {
        let sandbox = tempfile::tempdir().expect("Failed to create sandbox");
        let click_yml_node = ClickYml::new(PathBuf::from("clicks.yml"));
        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![];
        let result = click_yml_node.build(sandbox.path(), &predecessors);
        assert!(!result, "build should fail without predecessor");
    }
}
