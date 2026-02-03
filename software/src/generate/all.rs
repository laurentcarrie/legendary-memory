use crate::generate::generate::{generate_json_book, generate_json_song, mount_files};
use crate::model::use_model::World;
use crate::model::world::make;
use std::fs;
use std::path::PathBuf;

pub fn generate_all(
    songdir: PathBuf,
    bookdir: PathBuf,
    builddir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("generate all");
    fs::create_dir_all(&builddir)?;
    // let src_song_dir: PathBuf = Path::new(source_song_root).canonicalize().expect("root");
    // let src_book_dir: PathBuf = Path::new(source_book_root).canonicalize().expect("root");
    // let builddir: PathBuf = Path::new(buildroot).canonicalize().expect("buildroot");

    let world: World = make(&songdir, &bookdir, &builddir)?;

    // for song in &world.broken_songs {
    //     let path = song.0.to_str().unwrap_or("unknown path, huh");
    //     log::error!("¨{}:{} {:?} , {}", file!(), line!(), path, song.1);

    //     println!("path : {}", path);
    //     println!("error : {}", song.1);
    //     // return Err("exiting because of incorrect song.json".into());
    // }

    for song in &world.songs {
        generate_json_song(song)?;
    }

    mount_files(&world)?;

    for book in &world.books {
        // generate_main_book(book)?;
        generate_json_book(book)?;
    }

    // generate_song_omakefile(&world.songs[0])?;
    // generate_from_handlebars_templates(&world)?;

    // log::debug!("SUCCESS !");

    //dbg!(world);
    Ok(())
}
