use std::path::{Path, PathBuf};
use yamake::model::GNode;

/// Build node that decodes an MP3 click track, detects ticks (transient peaks),
/// and writes a YAML file with tick offsets relative to the first tick.
pub struct ClickYml {
    pub path: PathBuf,
}

impl ClickYml {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Minimum number of silent samples between two ticks to avoid double-counting
/// a single transient that spans multiple samples.
const MIN_GAP_SAMPLES: usize = 2000;

/// Fraction of the peak amplitude used as the detection threshold.
const THRESHOLD_RATIO: f32 = 0.3;

impl GNode for ClickYml {
    fn tag(&self) -> String {
        "clicks.yml".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Find the single MP3 predecessor
        let mp3_predecessors: Vec<_> = predecessors.iter().filter(|p| p.tag() == "mp3").collect();
        if mp3_predecessors.len() != 1 {
            log::error!(
                "ClickYml {} expected exactly 1 mp3 predecessor, got {}",
                self.path.display(),
                mp3_predecessors.len()
            );
            return false;
        }

        let mp3_path = sandbox.join(mp3_predecessors[0].pathbuf());
        let mp3_data = match std::fs::read(&mp3_path) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to read MP3 file {}: {}", mp3_path.display(), e);
                return false;
            }
        };

        // Decode MP3 to PCM samples
        let mut decoder = minimp3::Decoder::new(std::io::Cursor::new(mp3_data));
        let mut samples: Vec<f32> = Vec::new();
        let mut sample_rate = 0u32;

        loop {
            match decoder.next_frame() {
                Ok(frame) => {
                    if sample_rate == 0 {
                        sample_rate = frame.sample_rate as u32;
                    }
                    // Mix channels down to mono
                    let channels = frame.channels;
                    for chunk in frame.data.chunks(channels) {
                        let mono: f32 =
                            chunk.iter().map(|&s| s as f32).sum::<f32>() / channels as f32;
                        samples.push(mono);
                    }
                }
                Err(minimp3::Error::Eof) => break,
                Err(e) => {
                    log::error!("Error decoding MP3 {}: {:?}", mp3_path.display(), e);
                    return false;
                }
            }
        }

        if samples.is_empty() || sample_rate == 0 {
            log::error!("No audio data decoded from {}", mp3_path.display());
            return false;
        }

        // Detect ticks via amplitude threshold
        let peak = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);

        if peak == 0.0 {
            log::error!("Silent audio in {}", mp3_path.display());
            return false;
        }

        let threshold = peak * THRESHOLD_RATIO;
        let mut tick_indices: Vec<usize> = Vec::new();

        for (i, &s) in samples.iter().enumerate() {
            if s.abs() > threshold {
                // Only register a new tick if we are past the minimum gap from the last one
                if tick_indices
                    .last()
                    .map_or(true, |&last| i - last >= MIN_GAP_SAMPLES)
                {
                    tick_indices.push(i);
                }
            }
        }

        if tick_indices.is_empty() {
            log::error!("No ticks detected in {}", mp3_path.display());
            return false;
        }

        // Compute offsets relative to the first tick
        let first = tick_indices[0];
        let ticks: Vec<f64> = tick_indices
            .iter()
            .map(|&idx| (idx - first) as f64 / sample_rate as f64)
            .collect();

        // Serialize to YAML
        #[derive(serde::Serialize)]
        struct ClickData {
            ticks: Vec<f64>,
        }

        let yaml = match serde_yaml::to_string(&ClickData { ticks }) {
            Ok(y) => y,
            Err(e) => {
                log::error!("Failed to serialize click data: {}", e);
                return false;
            }
        };

        // Write output
        let out_path = sandbox.join(&self.path);
        if let Some(parent) = out_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!(
                    "Failed to create directory for {}: {}",
                    self.path.display(),
                    e
                );
                return false;
            }
        }

        if let Err(e) = std::fs::write(&out_path, &yaml) {
            log::error!("Failed to write {}: {}", out_path.display(), e);
            return false;
        }

        log::info!(
            "Detected {} ticks in {}, wrote {}",
            tick_indices.len(),
            mp3_path.display(),
            out_path.display()
        );

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Mp3;

    #[test]
    fn test_click_yml_build() {
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        // Copy the test MP3 fixture into the sandbox
        let mp3_rel = PathBuf::from("click/clicks.mp3");
        let mp3_sandbox = sandbox.path().join(&mp3_rel);
        std::fs::create_dir_all(mp3_sandbox.parent().unwrap()).unwrap();
        std::fs::copy("tests/data/click/clicks.mp3", &mp3_sandbox).unwrap();

        // Set up nodes
        let mp3_node = Mp3::new(mp3_rel);
        let click_node = ClickYml::new(PathBuf::from("click/clicks.yml"));

        // Build with the mp3 as predecessor
        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![&mp3_node];
        let ok = click_node.build(sandbox.path(), &predecessors);
        assert!(ok, "build() should succeed");

        // Read and parse the output YAML
        let out_path = sandbox.path().join("click/clicks.yml");
        assert!(out_path.exists(), "click.yml should be written");

        #[derive(serde::Deserialize)]
        struct ClickData {
            ticks: Vec<f64>,
        }

        let yaml = std::fs::read_to_string(&out_path).unwrap();
        let data: ClickData = serde_yaml::from_str(&yaml).unwrap();

        // The fixture has 4 ticks at 0s, 0.5s, 1.0s, 1.5s
        assert_eq!(data.ticks.len(), 4, "should detect 4 ticks");
        assert_eq!(data.ticks[0], 0.0, "first tick should be at offset 0");

        // Check intervals are approximately 0.5s (allow MP3 encoding tolerance)
        for i in 1..data.ticks.len() {
            let interval = data.ticks[i] - data.ticks[i - 1];
            assert!(
                (interval - 0.5).abs() < 0.05,
                "interval {} ({:.4}s) should be ~0.5s",
                i,
                interval
            );
        }
    }

    #[test]
    fn test_click_yml_no_mp3_predecessor() {
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");
        let click_node = ClickYml::new(PathBuf::from("click/clicks.yml"));

        // Build with no predecessors should fail
        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![];
        let ok = click_node.build(sandbox.path(), &predecessors);
        assert!(!ok, "build() should fail with no mp3 predecessor");
    }
}
