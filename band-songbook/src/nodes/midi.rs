use std::path::{Path, PathBuf};
use std::process::Command;
use yamake::model::GNode;

/// Build node for the MIDI output produced by LilyPond compilation.
///
/// `lilypond-book` writes its MIDI into a content-hashed subdirectory of
/// `<stem>.output`, which is awkward to depend on. This node instead runs
/// `lilypond` directly on the source so the MIDI lands at a predictable
/// `<stem>.midi` next to it.
pub struct MidiOfLilypond {
    pub path: PathBuf,
}

impl MidiOfLilypond {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GNode for MidiOfLilypond {
    fn tag(&self) -> String {
        "midi".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn build(&self, sandbox: &Path, predecessors: &[&(dyn GNode + Send + Sync)]) -> bool {
        let ly_predecessors: Vec<_> = predecessors
            .iter()
            .filter(|p| p.tag() == "lilypond")
            .collect();

        if ly_predecessors.len() != 1 {
            log::error!(
                "MidiOfLilypond {} expects exactly one lilypond predecessor, found {}",
                self.path.display(),
                ly_predecessors.len()
            );
            return false;
        }

        let ly_pathbuf = ly_predecessors[0].pathbuf();
        let ly_filename = match ly_pathbuf.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => {
                log::error!("MidiOfLilypond predecessor has no file name");
                return false;
            }
        };

        // The MIDI is written next to the source, so lilypond runs in the song
        // directory: that is also where `\include "macros.ly"` resolves from.
        let workdir = sandbox.join(self.path.parent().unwrap_or(Path::new("")));
        let stem = match self.path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => {
                log::error!(
                    "MidiOfLilypond path has no file stem: {}",
                    self.path.display()
                );
                return false;
            }
        };

        log::info!(
            "Running lilypond for MIDI on {} in {}",
            ly_filename,
            workdir.display()
        );

        let mut cmd = Command::new("lilypond");
        cmd.arg("-dno-print-pages")
            .arg("-o")
            .arg(stem)
            .arg(ly_filename)
            .current_dir(&workdir);

        let node_id = self.path.to_string_lossy();
        let stdout_path = sandbox.join("logs").join(format!("{}.stdout", &node_id));
        let stderr_path = sandbox.join("logs").join(format!("{}.stderr", &node_id));
        let _ = std::fs::create_dir_all(stdout_path.parent().unwrap_or(Path::new("")));

        match cmd.output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let _ = std::fs::write(&stdout_path, &stdout);
                let _ = std::fs::write(&stderr_path, &stderr);

                if !out.status.success() {
                    log::error!(
                        "lilypond failed for {} ({ly_filename})",
                        self.path.display()
                    );
                    if !stderr.is_empty() {
                        log::error!("lilypond stderr: {stderr}");
                    }
                    return false;
                }

                // A .ly with no \midi block compiles happily but writes nothing;
                // say so rather than letting the render node fail on a missing file.
                if !sandbox.join(&self.path).is_file() {
                    log::error!(
                        "lilypond produced no MIDI for {ly_filename} - does it have a \\midi block?"
                    );
                    return false;
                }
                true
            }
            Err(e) => {
                log::error!("Failed to run lilypond: {e} for {}", self.path.display());
                let _ = std::fs::write(&stderr_path, format!("Failed to run lilypond: {e}"));
                false
            }
        }
    }
}
