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

/// Discovers all songs in the given directory and builds them all.
/// Returns (success, graph) where success is true if all builds succeeded.
pub fn make_all(srcdir: &Path, sandbox: &Path) -> (bool, G) {
    let songs = discover(srcdir);
    let mut g = G::new(srcdir.to_path_buf(), sandbox.to_path_buf());

    // Add settings.yml as root node if it exists (so it gets copied to sandbox)
    let colors_path = srcdir.join("settings.yml");
    if colors_path.exists() {
        let colors_node = TexFile::new("settings.yml".into());
        let _ = g.add_root_node(colors_node);
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
                        let lyrics_path = parent_dir.join("lyrics").join(format!("{}.tex", item.id));
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
