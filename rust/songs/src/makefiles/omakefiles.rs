use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::{Song, World};
use crate::helpers::helpers::pdfname_of_song;

pub fn generate_refresh_sh(exepath: &PathBuf, world: &World) -> Result<(), Error> {
    println!("generate refresh.sh in {}", world.builddir.display());
    let mut p: PathBuf = world.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("refresh.sh");
    let mut output = File::create(p)?;
    write!(
        output,
        r###"
#!/bin/bash

set -e
set -x

{exepath} {srcdir} {builddir}

    "###,
        exepath = exepath.display(),
        srcdir = world.srcdir.display(),
        builddir = world.builddir.display()
    )?;

    Ok(())
}

pub fn generate_song_omakefile(song: &Song) -> Result<(), Error> {
    println!("generate Omakefile in {}", song.builddir.display());
    let mut p: PathBuf = song.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    let pdfname = pdfname_of_song(&song);
    p.push("OMakefile");
    let mut output = File::create(p)?;
    // dbg!(&output);
    // {
    //     Ok(x) => (x),
    //     Err(e) => return Err(e),
    // };
    // // write!(output, "Rust\n💖\nFun");

    write!(
        output,
        ".PHONY: pdf wav midi clean
clean:
\tbash  $(buildroot)/make_clean.sh
\trm -rf  "
    )?;

    for f in &song.lilypondfiles {
        write!(output, " {f} ")?;
    }

    write!(
        output,
        "

pdf : {pdfname}.pdf

main.pdf : main.tex"
    )?;

    for section in song.sections.iter() {
        write!(output, " mps/{name}-0.mps ", name = section.name)?;
    }

    for f in &song.lilypondfiles {
        let f2 = f.replace(".ly", "");
        write!(output, " {f2}.output/{f2}.tex ", f2 = f2)?
    }

    for f in &song.texfiles {
        write!(output, " {f} ")?
    }

    // mps/main-0.mps
    write!(
        output,
        "
\tbash $(buildroot)/make_pdf.sh main

{pdfname}.pdf : main.pdf
\tcp main.pdf $@
"
    )?;

    for section in song.sections.iter() {
        write!(
            output,
            r###"
mps/{name}-0.mps  : {name}.mp
    mkdir -p mps
    bash $(buildroot)/make_mpost.sh {name}.mp

"###,
            name = section.name
        )?;
    }

    for f in &song.lilypondfiles {
        write!(
            output,
            "
{name}.output/{name}.tex : {name}.ly
\tbash $(buildroot)/make_lytex.sh {name}

",
            name = f
        )?;
    }

    for (index, _section) in song.sections.iter().enumerate() {
        write!(
            output,
            r###"
mps/main-{index}.mps  : main.mp
    mkdir -p mps
    bash $(buildroot)/make_mpost.sh main.mp

"###,
            index = index
        )?;
    }

    for f in &song.lilypondfiles {
        let f2 = f.replace(".ly", "");
        write!(
            output,
            "
{f2}.output/{f2}.tex : {f}
\tbash $(buildroot)/make_lytex.sh {f2}

",
            f2 = f2,
            f = f
        )?;
    }

    write!(output, "midi : ")?;
    for w in &song.wavfiles {
        let name = w.replace(".wav", "");
        write!(output, " {name}.midi ", name = name)?;
    }
    writeln!(output, "")?;

    for w in &song.wavfiles {
        let name = w.replace(".wav", "");
        write!(
            output,
            "
wav : {name}.wav

midi : {name}.midi

{name}.wav {name}.midi : {name}.ly
\tbash $(buildroot)/make_wav.sh {name}
",
            name = name
        )?;
    }

    //
    // mps/main-0.mps  : main.mp
    // mkdir -p mps
    // bash $(buildroot)/make_mpost.sh main.mp
    //
    // intro.output/intro.tex : intro.ly
    // bash $(buildroot)/make_lytex.sh intro
    //
    // intro2.output/intro2.tex : intro2.ly
    // bash $(buildroot)/make_lytex.sh intro2
    //

    Ok(())
}
