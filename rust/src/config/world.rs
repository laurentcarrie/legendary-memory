use crate::config::config::{decode_book, decode_song};
use crate::config::get_config_files::{get_book_json_paths, get_song_json_paths};
use crate::config::model::{Book, Song, World};
use std::path::PathBuf;
pub fn make(srcdir: &PathBuf, srcbookdir: &PathBuf, builddir: &PathBuf) -> World {
    let songs: Vec<Song> = get_song_json_paths(&srcdir)
        .iter()
        .map(|p| decode_song(&builddir, &p).ok())
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
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
    };
    // dbg!(&world);
    world
}
