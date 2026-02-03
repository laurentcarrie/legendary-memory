pub mod chords;
pub mod discover;
pub mod helpers;
pub mod model;
pub mod nodes;
pub mod settings;

pub use discover::discover;

use model::{SectionItem, Song};
use nodes::{PdfFile, SongYml, TexFile};
use std::path::Path;

pub use yamake::model::G;

/// Checks if pattern is a subsequence of text (fuzzy match).
/// Each character in pattern must appear in text in order, but not necessarily contiguously.
fn fuzzy_match(text: &str, pattern: &str) -> bool {
    let mut text_chars = text.chars().peekable();
    for pat_char in pattern.chars() {
        loop {
            match text_chars.next() {
                Some(c) if c == pat_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

/// Discovers all songs in the given directory and builds them all.
/// Returns (success, graph) where success is true if all builds succeeded.
/// If settings_path is provided, it will be copied to sandbox/settings.yml.
/// If pattern is provided, only songs matching the pattern will be built.
pub fn make_all(
    srcdir: &Path,
    sandbox: &Path,
    settings_path: Option<&Path>,
    pattern: Option<&str>,
) -> (bool, G) {
    let songs = discover(srcdir);
    let mut g = G::new(srcdir.to_path_buf(), sandbox.to_path_buf());

    // Copy settings.yml to sandbox if provided
    if let Some(settings) = settings_path {
        let dest = sandbox.join("settings.yml");
        if let Err(e) = std::fs::copy(settings, &dest) {
            log::error!("Failed to copy settings.yml to sandbox: {e}");
        }
    }

    if songs.is_empty() {
        return (true, g);
    }

    for song_path in songs {
        // Convert absolute path to relative path from srcdir
        let rel_path = match song_path.strip_prefix(srcdir) {
            Ok(p) => p.to_path_buf(),
            Err(_) => continue,
        };

        let parent_dir = rel_path.parent().unwrap_or(Path::new(""));

        // Read song.yml to get structure for lyrics files
        let song: Option<Song> = std::fs::read_to_string(&song_path)
            .ok()
            .and_then(|content| serde_yaml::from_str(&content).ok());

        // Filter by pattern if provided (fuzzy match against author + title)
        if let Some(pat) = pattern {
            let matches = song.as_ref().is_some_and(|s| {
                let search_str = format!("{} {}", s.info.author, s.info.title).to_lowercase();
                fuzzy_match(&search_str, &pat.to_lowercase())
            });
            if !matches {
                continue;
            }
        }

        // Create SongYml node
        let song_node = SongYml::new(rel_path.clone());
        let song_idx = match g.add_root_node(song_node) {
            Ok(idx) => idx,
            Err(_) => continue,
        };

        // Add body.tex as root node (source file from srcdir, not generated)
        let body_path = parent_dir.join("body.tex");
        let body_node = TexFile::new(body_path);
        let _ = g.add_root_node(body_node);

        // Add lyrics files as root nodes for each Chords and Ref section
        if let Some(ref song_data) = song {
            for item in &song_data.structure {
                match &item.item {
                    SectionItem::Chords(_) | SectionItem::Ref(_) => {
                        let lyrics_path =
                            parent_dir.join("lyrics").join(format!("{}.tex", item.id));
                        let lyrics_node = TexFile::new(lyrics_path);
                        let _ = g.add_root_node(lyrics_node);
                    }
                    _ => {}
                }
            }
        }

        // Create matching PdfFile node (main.pdf in same directory)
        let pdf_path = parent_dir.join("main.pdf");
        let pdf_node = PdfFile::new(pdf_path);
        let pdf_idx = match g.add_node(pdf_node) {
            Ok(idx) => idx,
            Err(_) => continue,
        };

        g.add_edge(song_idx, pdf_idx);
    }

    let success = g.make();
    (success, g)
}

#[cfg(test)]
mod tests;
