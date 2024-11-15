use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::World;
use crate::helpers::helpers::pdfname_of_song;

pub fn f() {}

pub fn generate_omakeroot(world: &World) -> Result<(), Error> {
    log::debug!("generate OMakeroot in {}", world.builddir.display());
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
    log::debug!("generate OMakefile in {}", world.builddir.display());
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

    write!(output, "delivery:\\\n")?;
    for song in &world.songs {
        write!(
            output,
            "{}",
            format!(
                "\t{p}/{pdfname}.pdf \\\n",
                p = song.builddir.display(),
                pdfname = pdfname_of_song(&song)
            )
        )?;
    }
    write!(
        output,
        "
\trm -rf delivery
\tmkdir delivery
\tcp $^ delivery/.
"
    )?;
    Ok(())
}
