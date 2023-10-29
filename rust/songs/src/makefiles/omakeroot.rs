use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::World;

pub fn f() {}

pub fn generate_omakeroot(world: &World) -> Result<(), Error> {
    println!("generate OMakeroot in {}", world.builddir.display());
    let mut p: PathBuf = world.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakeroot");
    let mut output = File::create(p)?; //.expect("file created");
                                       // write!(output, "Rust\n💖\nFun");

    write!(output, "srcdir = {} \n", world.srcdir.display())?;

    // prefix = delivery
    write!(output, "buildroot = {}\n", world.builddir.display())?;
    // buildroot = /home/laurent/work/legendary-memory/build-songs
    write!(output, "DefineCommandVars()\n")?;
    write!(output, "public.srcdir = $(dir $(srcdir))\n")?;
    write!(output, "CREATE_SUBDIRS=true\n")?;
    write!(output, "vmount(-c,$(srcdir),songs)\n")?;
    write!(output, "prefix=delivery\n")?;
    write!(output, "mkdir -p $(prefix)\n")?;
    write!(output, ".SUBDIRS: .\n")?;
    Ok(())
}

pub fn generate_root_omakefile(world: &World) -> Result<(), Error> {
    println!("generate OMakefile in {}", world.builddir.display());
    let mut p: PathBuf = world.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakefile");
    let mut output = File::create(p)?; //.expect("file created");
                                       // write!(output, "Rust\n💖\nFun");

    let mut subdirs = HashSet::new();
    for song in &world.songs {
        subdirs.insert(&song.builddir);
    }

    write!(output, "# root is {}\n", world.builddir.display())?;

    write!(
        output,
        "
.PHONY: all install clean pdf delivery clean

.SUBDIRS: \\
"
    )?;

    for subdir in subdirs {
        write!(output, "\t{} \\\n", subdir.display())?;
    }

    write!(
        output,
        "

"
    )?;

    // songs/eiffel/la_rue \
    // songs/jolene/au_conditionnel \
    // songs/pixies/gouge_away \
    // songs/aerosmith/crazy \
    // songs/matmatah/au_conditionnel \
    // songs/luke/la_sentinelle \
    // songs/estelle/american_boy \
    // songs/acdc/you-shook-me-all-night-long \
    // songs/red_hot_chili_peppers/can_t_stop \
    // songs/placebo/special_k \
    // songs/dionysos/song_for_jedi \
    // songs/deep_purple/smoke_on_the_water \
    //
    // delivery:\
    // songs/deep_purple/smoke_on_the_water/deep-purple--@--smoke-on-the-water.pdf \
    // songs/dionysos/song_for_jedi/dionysos--@--song-for-jedi.pdf \

    Ok(())
}
