use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};

//use crate::config::Song::decode_Song;
use crate::config::model::World;
use crate::config::world::make;
use crate::generated::generate::generate;
// use crate::makefiles::omakeroot::generate_omakeroot ;
use crate::emitter::xxx::fff;
use crate::makefiles::omakefiles::{generate_refresh_sh, generate_song_omakefile};
use crate::makefiles::omakeroot::{generate_omakeroot, generate_root_omakefile};

pub mod config;
pub mod emitter;
pub mod generated;
pub mod helpers;
pub mod makefiles;
fn main() -> Result<(), Error> {
    fff();
    let args: Vec<String> = env::args().collect();
    let root = &args[1];
    let buildroot = &args[2];
    let _ = fs::create_dir_all(&buildroot)?;
    let exepath: PathBuf = Path::new(&args[0]).canonicalize().expect("exepath");
    let srcdir: PathBuf = Path::new(root).canonicalize().expect("root");
    let builddir: PathBuf = Path::new(buildroot).canonicalize().expect("buildroot");
    let _ = fs::create_dir_all(&buildroot)?;

    // dbg!(root2);
    // let p : PathBuf = PathBuf::from("/the/head");
    // let s = p.into_os_string() ;
    // let s = root2.into_os_string() ;

    let world: World = make(&srcdir, &builddir);
    generate_refresh_sh(&exepath, &world)?;
    generate_omakeroot(&world)?;
    generate_root_omakefile(&world)?;
    // world
    //     .songs
    //     .iter()
    //     .for_each(|s|  generate_song_omakefile(&world, &s)? )?;

    for song in &world.songs {
        generate_song_omakefile(&song)?;
    }

    generate_song_omakefile(&world.songs[0])?;
    generate(&world)?;

    // println!("SUCCESS !");

    //dbg!(world);
    Ok(())
}
