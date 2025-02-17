use crate::config::config::{decode_book, decode_song};
use crate::config::get_config_files::{get_book_json_paths, get_song_json_paths};
use crate::config::input_model::{UserBookWithPath, UserSongWithPath, UserWorld};
use crate::config::model::{Book, Section, Song, World};
use serde_json;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
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
    let songs_zip: Vec<(Song, UserSongWithPath)> = get_song_json_paths(&srcdir)
        .iter()
        .filter_map(|p| match decode_song(&builddir, &sections, &p) {
            Ok(p) => Some(p),
            Err(e) => {
                log::error!("in {} : {}", p.display(), e);
                None
            }
        })
        .collect();
    let songs: Vec<_> = songs_zip.iter().map(|s| s.0.clone()).collect();

    let usongs_with_path: Vec<_> = songs_zip.iter().map(|s| s.1.clone()).collect();

    let books_zip: Vec<(Book, UserBookWithPath)> = get_book_json_paths(&srcbookdir)
        .iter()
        .map(|p| decode_book(&builddir, &p).ok())
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect();
    let books: Vec<_> = books_zip.iter().map(|s| s.0.clone()).collect();
    let ubooks_with_path: Vec<_> = books_zip.iter().map(|s| s.1.clone()).collect();

    let world = World {
        builddir: builddir.to_path_buf(),
        srcdir: srcdir.to_path_buf(),
        songs: songs,
        books: books,
        sections: sections,
    };
    {
        let data = {
            let user_world = UserWorld {
                songs: usongs_with_path,
                books: ubooks_with_path,
            };
            serde_json::to_string(&user_world).unwrap()
        };
        let mut path = builddir.clone();
        path.push("world.json");
        let mut file = File::create(&path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    // dbg!(&world);
    world
}
