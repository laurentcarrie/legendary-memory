use std::path::{Path, PathBuf};
use yamake::model::GNode;

use crate::model::ClicksDefinition;

/// Build node that overlays click sounds onto a song MP3 at tick positions
/// from `clicks.yml`, producing `overlay.mp3` for manual verification.
pub struct ClickCheckMp3 {
    pub path: PathBuf,
}

impl ClickCheckMp3 {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Generate a click sample: 1000 Hz sine wave, 10 ms, with linear decay envelope.
fn generate_click(sample_rate: u32) -> Vec<f32> {
    let num_samples = (0.01 * sample_rate as f64) as usize;
    (0..num_samples)
        .map(|i| {
            let t = i as f64 / sample_rate as f64;
            let sine = (2.0 * std::f64::consts::PI * 1000.0 * t).sin();
            let envelope = 1.0 - (i as f64 / num_samples as f64);
            (0.8 * sine * envelope) as f32
        })
        .collect()
}

impl GNode for ClickCheckMp3 {
    fn tag(&self) -> String {
        "click_check_mp3".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        // Find clicks.yml and mp3 predecessors
        let clicks_yml: Vec<_> = predecessors
            .iter()
            .filter(|p| p.tag() == "clicks.yml")
            .collect();
        let mp3s: Vec<_> = predecessors.iter().filter(|p| p.tag() == "mp3").collect();

        if clicks_yml.len() != 1 {
            log::error!(
                "ClickCheckMp3 {} expected 1 clicks.yml predecessor, got {}",
                self.path.display(),
                clicks_yml.len()
            );
            return false;
        }
        if mp3s.len() != 1 {
            log::error!(
                "ClickCheckMp3 {} expected 1 mp3 predecessor, got {}",
                self.path.display(),
                mp3s.len()
            );
            return false;
        }

        // Read and parse clicks.yml
        let clicks_yml_path = sandbox.join(clicks_yml[0].pathbuf());
        let yaml = match std::fs::read_to_string(&clicks_yml_path) {
            Ok(y) => y,
            Err(e) => {
                log::error!("Failed to read {}: {}", clicks_yml_path.display(), e);
                return false;
            }
        };
        let clicks: ClicksDefinition = match serde_yaml::from_str(&yaml) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to parse {}: {}", clicks_yml_path.display(), e);
                return false;
            }
        };

        if clicks.ticks.is_empty() {
            log::error!("No ticks in {}", clicks_yml_path.display());
            return false;
        }

        // Decode song MP3 to PCM
        let mp3_path = sandbox.join(mp3s[0].pathbuf());
        let mp3_data = match std::fs::read(&mp3_path) {
            Ok(d) => d,
            Err(e) => {
                log::error!("Failed to read {}: {}", mp3_path.display(), e);
                return false;
            }
        };

        let mut decoder = minimp3::Decoder::new(std::io::Cursor::new(mp3_data));
        let mut song_samples: Vec<f32> = Vec::new();
        let mut sample_rate = 0u32;
        let mut channels = 1usize;

        loop {
            match decoder.next_frame() {
                Ok(frame) => {
                    if sample_rate == 0 {
                        sample_rate = frame.sample_rate as u32;
                        channels = frame.channels;
                    }
                    // Keep all channels (don't mix to mono)
                    for &s in &frame.data {
                        song_samples.push(s as f32 / 32768.0);
                    }
                }
                Err(minimp3::Error::Eof) => break,
                Err(e) => {
                    log::error!("Error decoding {}: {:?}", mp3_path.display(), e);
                    return false;
                }
            }
        }

        if song_samples.is_empty() || sample_rate == 0 {
            log::error!("No audio decoded from {}", mp3_path.display());
            return false;
        }

        // Generate click sound
        let click = generate_click(sample_rate);

        // Overlay clicks onto song samples at each tick position
        let total_frames = song_samples.len() / channels;
        for &tick in &clicks.ticks {
            let frame_idx = (tick * sample_rate as f64) as usize;
            for (i, &click_sample) in click.iter().enumerate() {
                let frame = frame_idx + i;
                if frame >= total_frames {
                    break;
                }
                // Add click to all channels
                for ch in 0..channels {
                    let sample_idx = frame * channels + ch;
                    if sample_idx < song_samples.len() {
                        song_samples[sample_idx] += click_sample;
                    }
                }
            }
        }

        // Clip to [-1, 1]
        for s in &mut song_samples {
            *s = s.clamp(-1.0, 1.0);
        }

        // Write WAV to temp file
        let temp_wav = match tempfile::NamedTempFile::new() {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to create temp WAV file: {}", e);
                return false;
            }
        };
        let temp_wav_path = temp_wav.path().to_path_buf();

        if !write_wav(&temp_wav_path, sample_rate, channels as u16, &song_samples) {
            return false;
        }

        // Create output directory
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

        // Convert WAV to MP3 with ffmpeg
        let status = std::process::Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                &temp_wav_path.to_string_lossy(),
                "-b:a",
                "192k",
                &out_path.to_string_lossy(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(s) => {
                log::error!("ffmpeg exited with status {}", s);
                return false;
            }
            Err(e) => {
                log::error!("Failed to run ffmpeg: {}", e);
                return false;
            }
        }

        log::info!(
            "Created overlay MP3 with {} ticks at {}",
            clicks.ticks.len(),
            out_path.display()
        );

        true
    }
}

/// Write PCM samples as a WAV file.
fn write_wav(path: &Path, sample_rate: u32, channels: u16, samples: &[f32]) -> bool {
    use std::io::Write;

    let bits_per_sample: u16 = 16;
    let byte_rate = sample_rate * channels as u32 * bits_per_sample as u32 / 8;
    let block_align = channels * bits_per_sample / 8;
    let data_size = samples.len() as u32 * 2; // 16-bit samples = 2 bytes each

    let mut file = match std::fs::File::create(path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("Failed to create WAV file {}: {}", path.display(), e);
            return false;
        }
    };

    let write_result = (|| -> std::io::Result<()> {
        // RIFF header
        file.write_all(b"RIFF")?;
        file.write_all(&(36 + data_size).to_le_bytes())?;
        file.write_all(b"WAVE")?;
        // fmt chunk
        file.write_all(b"fmt ")?;
        file.write_all(&16u32.to_le_bytes())?; // chunk size
        file.write_all(&1u16.to_le_bytes())?; // PCM format
        file.write_all(&channels.to_le_bytes())?;
        file.write_all(&sample_rate.to_le_bytes())?;
        file.write_all(&byte_rate.to_le_bytes())?;
        file.write_all(&block_align.to_le_bytes())?;
        file.write_all(&bits_per_sample.to_le_bytes())?;
        // data chunk
        file.write_all(b"data")?;
        file.write_all(&data_size.to_le_bytes())?;
        for &s in samples {
            let i16_val = (s * 32767.0) as i16;
            file.write_all(&i16_val.to_le_bytes())?;
        }
        Ok(())
    })();

    if let Err(e) = write_result {
        log::error!("Failed to write WAV file {}: {}", path.display(), e);
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{ClickYml, Mp3};

    #[test]
    fn test_click_check_mp3_build() {
        let srcdir = tempfile::tempdir().expect("create srcdir");
        let sandbox = tempfile::tempdir().expect("create sandbox");

        // Create clicks.yml in srcdir and sandbox
        let clicks_yml = "ticks:\n- 0.0\n- 0.5\n- 1.0\n- 1.5\n";
        std::fs::write(srcdir.path().join("clicks.yml"), clicks_yml).unwrap();
        std::fs::write(sandbox.path().join("clicks.yml"), clicks_yml).unwrap();

        // Copy test MP3 to sandbox as song.mp3
        std::fs::copy(
            "tests/data/click/clicks.mp3",
            sandbox.path().join("song.mp3"),
        )
        .unwrap();

        let clicks_node = ClickYml::new(PathBuf::from("clicks.yml"), srcdir.path()).unwrap();
        let mp3_node = Mp3::new(PathBuf::from("song.mp3"));
        let check_node = ClickCheckMp3::new(PathBuf::from("overlay.mp3"));

        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![&clicks_node, &mp3_node];
        let ok = check_node.build(sandbox.path(), &predecessors);
        assert!(ok, "build() should succeed");

        let out_path = sandbox.path().join("overlay.mp3");
        assert!(out_path.exists(), "overlay.mp3 should be created");
        assert!(
            std::fs::metadata(&out_path).unwrap().len() > 0,
            "overlay.mp3 should not be empty"
        );
    }

    #[test]
    fn test_click_check_mp3_missing_predecessor() {
        let sandbox = tempfile::tempdir().expect("create sandbox");
        let check_node = ClickCheckMp3::new(PathBuf::from("overlay.mp3"));

        // No predecessors
        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![];
        let ok = check_node.build(sandbox.path(), &predecessors);
        assert!(!ok, "build() should fail with no predecessors");
    }
}
