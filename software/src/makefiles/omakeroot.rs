use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::World;
use crate::helpers::helpers::pdfname_of_book;

pub fn f() {}

pub fn generate_omakeroot(world: &World) -> Result<(), Error> {
    log::debug!("generate OMakeroot in {}", world.builddir.display());
    let mut p: PathBuf = world.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakeroot");
    let mut output = File::create(p)?; //.expect("file created");
                                       // write!(output, "Rust\n💖\nFun");

    let body = String::from_utf8(include_bytes!("OMakeroot").to_vec()).expect("OMakeroot");
    let body = body.replace("@@@SONGS_DIR@@@", &world.srcdir.display().to_string());
    let body = body.replace("@@@BUILD_DIR@@@", &world.builddir.display().to_string());
    write!(output, "{}", body)?;
    // buildroot = /home/laurent/work/legendary-memory/build-songs
    // write!(output, "DefineCommandVars()\n")?;
    // write!(output, "public.srcdir = $(dir $(srcdir))\n")?;
    // write!(output, "CREATE_SUBDIRS=true\n")?;
    // write!(output, "vmount(-c,$(srcdir),songs)\n")?;
    // write!(output, "prefix=delivery\n")?;
    // write!(output, "mkdir -p $(prefix)\n")?;
    // write!(output, ".SUBDIRS: .\n")?;
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

    // assert_eq!(world.books.len() as i32, 1);
    for book in &world.books {
        subdirs.insert(&book.builddir);
    }

    write!(output, "# root is {}\n", world.builddir.display())?;

    write!(
        output,
        r###"
.PHONY: all install clean pdf delivery delivery_songs delivery_books clean gdrive

gdrive:
	bash $(buildroot)/make_gdrive.sh /zik/songs delivery

.SUBDIRS: \
"###
    )?;

    for subdir in subdirs {
        write!(output, "\t{} \\\n", subdir.display())?;
    }

    write!(
        output,
        "

"
    )?;

    write!(output, "delivery_songs:\\\n")?;
    for song in &world.songs {
        write!(
            output,
            "{}",
            format!(
                "\t{p}/{pdfname}.pdf \\\n",
                p = song.builddir.display(),
                pdfname = song.pdfname
            )
        )?;
    }
    write!(
        output,
        "
\tmkdir -p delivery
\tcp $^ delivery/.

"
    )?;

    write!(output, "delivery_books:\\\n")?;
    for book in &world.books {
        write!(
            output,
            "{}",
            format!(
                "\t{p}/{pdfname}.pdf \\\n",
                p = book.builddir.display(),
                pdfname = pdfname_of_book(&book)
            )
        )?;
    }
    write!(
        output,
        "
\tmkdir -p delivery
\tcp $^ delivery/.

"
    )?;

    write!(output, "delivery: delivery_books delivery_songs\n")?;

    Ok(())
}
