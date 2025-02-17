use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::errors::MyError;
use crate::generate::handlebars_helpers::get_handlebar;
use crate::model::model::Song;
use crate::model::model::World;

pub fn generate_song_omakefile(song: &Song) -> Result<(), Error> {
    log::debug!("generate Omakefile in {}", song.builddir.display());
    let mut p: PathBuf = song.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakefile");
    let mut output = File::create(p)?;
    let template =
        String::from_utf8(include_bytes!("../../others/makefiles/omakefile").to_vec()).unwrap();

    let mut texset: HashSet<String> = HashSet::new();
    for f in &song.lilypondfiles {
        texset.insert(PathBuf::from(f).set_extension("").to_string());
    }
    let mut wavset: HashSet<String> = HashSet::new();
    for f in &song.wavfiles {
        wavset.insert(PathBuf::from(f).set_extension("").to_string());
    }

    // let lilytexfiles = texset.difference(&wavset).collect();
    // let lilytexwavfiles = texset.union(&wavset).collect();
    // let lilywavfiles = wavset.difference(&texset).collect();

    let mut h = get_handlebar()?;
    h.register_template_string("t1", template).unwrap();
    let output_data = h.render("t1", song).unwrap();
    let _ = output.write(output_data.as_bytes()).unwrap();
    Ok(())
}

pub fn generate_root_omakefile(world: &World) -> Result<(), Error> {
    log::debug!("generate Omakefile in {}", world.builddir.display());
    let mut p: PathBuf = world.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakefile");
    let mut output = File::create(p)?;
    let template =
        String::from_utf8(include_bytes!("../../others/makefiles/root_omakefile").to_vec())
            .unwrap();

    let mut h = get_handlebar()?;
    h.register_template_string("t1", template).unwrap();
    let output_data = h.render("t1", world).unwrap();
    let _ = output.write(output_data.as_bytes()).unwrap();
    Ok(())
}

pub fn generate_omakeroot(world: &World) -> Result<(), MyError> {
    log::debug!("generate Omakefile in {}", world.builddir.display());
    let mut p: PathBuf = world.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakeroot");
    let mut output = File::create(p)?;
    let template = String::from_utf8(include_bytes!("../../others/makefiles/omakeroot").to_vec())?;
    let mut h = get_handlebar()?;
    h.register_template_string("t1", template)?;
    let output_data = h.render("t1", world)?;
    let _ = output.write(output_data.as_bytes())?;
    Ok(())
}
