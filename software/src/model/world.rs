use crate::model::config::{decode_book, decode_song};
use crate::model::get_config_files::{get_book_json_paths, get_song_json_paths};
use crate::model::input_model::UserWorld;
use crate::model::model::{Section, World};
use serde_json;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

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

pub fn make(
    srcdir: &PathBuf,
    srcbookdir: &PathBuf,
    builddir: &Path,
) -> Result<World, Box<dyn std::error::Error>> {
    // the available song sections, for rendering
    let sections = get_sections();

    // for all song.json found in the songdir tree
    let (songs, broken_songs): (Vec<_>, Vec<_>) = get_song_json_paths(srcdir)
        .into_iter()
        .map(|p| (p.clone(), decode_song(builddir, &sections, &p)))
        .partition(|x| x.1.is_ok());

    let (songs, usongs_with_path): (Vec<_>, Vec<_>) =
        songs.into_iter().map(|x| x.1.unwrap()).unzip();
    let broken_songs: Vec<_> = broken_songs
        .into_iter()
        .map(|(p, e)| (p, e.err().unwrap().to_string()))
        .collect();

    let (books, broken_books): (Vec<_>, Vec<_>) = get_book_json_paths(srcbookdir)
        .into_iter()
        .map(|p| {
            (
                p.clone(),
                decode_book(srcdir, builddir, &p, &usongs_with_path),
            )
        })
        .partition(|x| x.1.is_ok());
    let (books, ubooks_with_path): (Vec<_>, Vec<_>) =
        books.into_iter().map(|s| s.1.unwrap()).unzip();
    let broken_books: Vec<_> = broken_books
        .into_iter()
        .map(|(p, e)| (p, e.err().unwrap().to_string()))
        .collect();

    let world = World {
        builddir: builddir.to_path_buf(),
        songdir: srcdir.to_path_buf(),
        bookdir: srcbookdir.to_path_buf(),
        songs,
        books,
        sections,
        broken_songs,
        broken_books,
    };
    {
        let data = {
            let user_world = UserWorld {
                songs: usongs_with_path,
                books: ubooks_with_path,
            };
            serde_json::to_string(&user_world).unwrap()
        };
        let mut path = builddir.to_path_buf();
        path.push("world.json");
        let mut file = File::create(&path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
    {
        let data = serde_json::to_string(&world)?;
        let mut path = builddir.to_path_buf();
        path.push("world-internal.json");
        std::fs::write(path.as_path(), data)?;
    }

    log::debug!("found {} song errors", world.broken_books.len());

    for e in world.broken_songs.iter() {
        log::error!("¨{}:{} {:?} , {}", file!(), line!(), e.0.to_str(), e.1);
    }

    for e in world.broken_books.iter() {
        log::error!("¨{}:{} {:?} , {}", file!(), line!(), e.0.to_str(), e.1);
    }
    // dbg!(&world);
    // if errors.is_empty() {
    Ok(world)
    // } else {
    //     let e = errors.first();
    //     let e = e.unwrap();
    //     let e = e.as_ref().err();
    //     let e = e.unwrap();
    //     let e = e.to_string();
    //     Err(MyError::MessageError(e))
    // }
}
