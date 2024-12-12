use crate::config::config::{decode_book, decode_song};
use crate::config::get_config_files::{get_book_json_paths, get_song_json_paths};
use crate::config::model::{Book, Section, World};
use serde_json;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn get_sections() -> BTreeMap<String, Section> {
    let data =
        String::from_utf8(include_bytes!("../../others/texfiles/sections.json").to_vec()).unwrap();
    let sections: Vec<Section> = serde_json::from_str(data.as_str()).unwrap();
    let mut map = BTreeMap::new();
    for s in sections {
        map.insert(s.id.clone(), s.clone());
    }
    map
}

pub fn make(srcdir: &PathBuf, srcbookdir: &PathBuf, builddir: &PathBuf) -> World {
    let sections = get_sections();
    let songs = get_song_json_paths(&srcdir)
        .iter()
        .filter_map(|p| match decode_song(&builddir, &sections, &p) {
            Ok(p) => Some(p),
            Err(e) => {
                log::error!("in {} : {}", p.display(), e);
                None
            }
        })
        .collect();

    let books: Vec<Book> = get_book_json_paths(&srcbookdir)
        .iter()
        .map(|p| decode_book(&builddir, &p).ok())
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect();

    let world = World {
        builddir: builddir.to_path_buf(),
        srcdir: srcdir.to_path_buf(),
        songs: songs,
        books: books,
        sections: sections,
    };
    // dbg!(&world);
    world
}
