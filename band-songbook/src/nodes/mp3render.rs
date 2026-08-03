use std::path::{Path, PathBuf};
use std::process::Command;
use yamake::model::GNode;

use crate::settings::Settings;

/// Standard locations for a General MIDI soundfont, tried in order when
/// neither `settings.yml` nor the environment names one.
const SOUNDFONT_CANDIDATES: &[&str] = &[
    "/usr/share/sounds/sf2/FluidR3_GM.sf2",
    "/usr/share/sounds/sf2/default-GM.sf2",
    "/usr/share/soundfonts/FluidR3_GM.sf2",
    "/usr/share/soundfonts/default.sf2",
    "/usr/share/sounds/sf3/FluidR3_GM.sf3",
];

/// Build node for an `.mp3` render of a song section: `fluidsynth` synthesises
/// the LilyPond MIDI, then `ffmpeg` encodes it.
///
/// The tag is `mp3render`, not `mp3`, so this is never mistaken for the song's
/// own `song.mp3` source node.
pub struct Mp3Render {
    pub path: PathBuf,
}

impl Mp3Render {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Resolve the soundfont to synthesise with: `soundfont` in settings.yml
    /// first, then `BAND_SONGBOOK_SOUNDFONT`, then the usual system paths.
    fn soundfont(sandbox: &Path) -> Option<PathBuf> {
        if let Ok(settings) = Settings::load(sandbox) {
            if let Some(sf) = settings.soundfont.filter(|s| !s.is_empty()) {
                let path = PathBuf::from(sf);
                if path.is_file() {
                    return Some(path);
                }
                log::warn!(
                    "settings.yml names soundfont {} but it does not exist - falling back",
                    path.display()
                );
            }
        }

        if let Ok(sf) = std::env::var("BAND_SONGBOOK_SOUNDFONT") {
            let path = PathBuf::from(&sf);
            if path.is_file() {
                return Some(path);
            }
            log::warn!(
                "BAND_SONGBOOK_SOUNDFONT is set to {sf} but it does not exist - falling back"
            );
        }

        SOUNDFONT_CANDIDATES
            .iter()
            .map(PathBuf::from)
            .find(|p| p.is_file())
    }

    /// Run a command, tee its output to `sandbox/logs`, and report success.
    fn run(cmd: &mut Command, tool: &str, node_id: &str, sandbox: &Path) -> bool {
        let stdout_path = sandbox
            .join("logs")
            .join(format!("{node_id}.{tool}.stdout"));
        let stderr_path = sandbox
            .join("logs")
            .join(format!("{node_id}.{tool}.stderr"));
        let _ = std::fs::create_dir_all(stdout_path.parent().unwrap_or(Path::new("")));

        match cmd.output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let _ = std::fs::write(&stdout_path, &stdout);
                let _ = std::fs::write(&stderr_path, &stderr);

                if !out.status.success() {
                    log::error!("{tool} failed for {node_id}");
                    if !stderr.is_empty() {
                        log::error!("{tool} stderr: {stderr}");
                    }
                    return false;
                }
                true
            }
            Err(e) => {
                log::error!("Failed to run {tool}: {e} for {node_id} - is {tool} installed?");
                let _ = std::fs::write(&stderr_path, format!("Failed to run {tool}: {e}"));
                false
            }
        }
    }
}

impl GNode for Mp3Render {
    fn tag(&self) -> String {
        "mp3render".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        let midi_predecessors: Vec<_> = predecessors.iter().filter(|p| p.tag() == "midi").collect();

        if midi_predecessors.len() != 1 {
            log::error!(
                "Mp3Render {} expects exactly one midi predecessor, found {}",
                self.path.display(),
                midi_predecessors.len()
            );
            return false;
        }

        let midi_path = sandbox.join(midi_predecessors[0].pathbuf());
        let mp3_path = sandbox.join(&self.path);
        // fluidsynth cannot write mp3, so synthesise to a scratch wav first and
        // encode from that. Kept beside the target, removed either way.
        let wav_path = mp3_path.with_extension("render.wav");

        let soundfont = match Self::soundfont(sandbox) {
            Some(sf) => sf,
            None => {
                log::error!(
                    "No soundfont found for {} - install one (e.g. `apt install fluid-soundfont-gm`), \
                     set `soundfont:` in settings.yml, or set BAND_SONGBOOK_SOUNDFONT",
                    self.path.display()
                );
                return false;
            }
        };

        log::info!(
            "Rendering {} from {} with {}",
            self.path.display(),
            midi_path.display(),
            soundfont.display()
        );

        let node_id = self.path.to_string_lossy().to_string();

        let mut synth = Command::new("fluidsynth");
        synth
            .arg("-ni")
            .arg("-F")
            .arg(&wav_path)
            .arg("-r")
            .arg("44100")
            .arg(&soundfont)
            .arg(&midi_path);

        if !Self::run(&mut synth, "fluidsynth", &node_id, sandbox) {
            let _ = std::fs::remove_file(&wav_path);
            return false;
        }

        // fluidsynth exits 0 even when it wrote nothing usable.
        if !wav_path.is_file() {
            log::error!("fluidsynth wrote no output for {}", self.path.display());
            return false;
        }

        let mut encode = Command::new("ffmpeg");
        encode
            .arg("-y")
            .arg("-loglevel")
            .arg("error")
            .arg("-i")
            .arg(&wav_path)
            .arg("-codec:a")
            .arg("libmp3lame")
            .arg("-q:a")
            .arg("2")
            .arg(&mp3_path);

        let encoded = Self::run(&mut encode, "ffmpeg", &node_id, sandbox);
        let _ = std::fs::remove_file(&wav_path);

        if !encoded {
            return false;
        }

        if !mp3_path.is_file() {
            log::error!("ffmpeg wrote no output for {}", self.path.display());
            return false;
        }
        true
    }
}
