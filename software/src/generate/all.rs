use crate::config::model::World;
use crate::config::world::make;
use crate::errors::MyError;
use crate::generate::generate::generate;
use crate::makefiles::omakefile_book::{generate_book_omakefile, generate_main_book};
use crate::makefiles::omakefile_song::{
    generate_omakeroot, generate_root_omakefile, generate_song_omakefile,
};
use std::fs;
use std::path::PathBuf;

pub fn generate_all(songdir: PathBuf, bookdir: PathBuf, builddir: PathBuf) -> Result<(), MyError> {
    // fff();
    let _ = fs::create_dir_all(&builddir)?;
    // let src_song_dir: PathBuf = Path::new(source_song_root).canonicalize().expect("root");
    // let src_book_dir: PathBuf = Path::new(source_book_root).canonicalize().expect("root");
    // let builddir: PathBuf = Path::new(buildroot).canonicalize().expect("buildroot");

    let world: World = make(&songdir, &bookdir, &builddir);
    generate_omakeroot(&world)?;
    generate_root_omakefile(&world)?;
    for song in &world.songs {
        generate_song_omakefile(&song)?;
    }

    for book in &world.books {
        generate_book_omakefile(&book)?;
        generate_main_book(&book)?;
    }

    // generate_song_omakefile(&world.songs[0])?;
    generate(&world)?;

    // log::debug!("SUCCESS !");

    //dbg!(world);
    Ok(())
}
