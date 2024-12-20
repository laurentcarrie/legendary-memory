use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::Song;
use crate::generate::handlebars_helpers::get_handlebar;

// pub fn generate_refresh_sh(exepath: &PathBuf, world: &World) -> Result<(), Error> {
//     log::debug!("generate refresh.sh in {}", world.builddir.display());
//     let mut p: PathBuf = world.builddir.clone();
//     let _ = fs::create_dir_all(&p)?;
//     p.push("refresh.sh");
//     let mut output = File::create(p)?;
//     write!(
//         output,
//         r###"
// #!/bin/bash
//
// set -e
// set -x
//
// {exepath} {srcdir} {builddir}
//
//     "###,
//         exepath = exepath.display(),
//         srcdir = world.srcdir.display(),
//         builddir = world.builddir.display()
//     )?;
//
//     Ok(())
// }

// pub fn generate_song_omakefile_old(song: &Song) -> Result<(), Error> {
//     log::debug!("generate Omakefile in {}", song.builddir.display());
//     let mut p: PathBuf = song.builddir.clone();
//     let _ = fs::create_dir_all(&p)?;
//     let pdfname = "blahblahblah".to_string();
//     p.push("OMakefile");
//     let mut output = File::create(p)?;
//     // dbg!(&output);
//     // {
//     //     Ok(x) => (x),
//     //     Err(e) => return Err(e),
//     // };
//     // // write!(output, "Rust\n💖\nFun");
//
//     write!(
//         output,
//         ".PHONY: pdf wav midi clean
// clean:
// \tbash  $(buildroot)/make_clean.sh
// \trm -rf  "
//     )?;
//
//     for f in &song.lilypondfiles {
//         write!(output, " {f} ")?;
//     }
//
//     write!(
//         output,
//         r###"
// pdf : {pdfname}.pdf
//
// main.tex : $(buildroot)/songs/main.tex
//     cp $(buildroot)/songs/main.tex .
//
// preamble.tex : $(buildroot)/songs/preamble.tex
//     cp $(buildroot)/songs/preamble.tex .
//
// chords.tex : $(buildroot)/songs/chords.tex
//     cp $(buildroot)/songs/chords.tex .
//
//
// #data.tex : data-utf8.tex
// #    iconv -f UTF-8 -t ISO-8859-15 data-utf8.tex > data.tex
//
// main.pdf : main.tex preamble.tex chords.tex body.tex data.tex "###
//     )?;
//
//     for f in &song.lilypondfiles {
//         let f2 = f.replace(".ly", "");
//         write!(output, " {f2}.output/{f2}.tex ", f2 = f2)?
//     }
//
//     for f in &song.texfiles {
//         write!(output, " {f} ")?
//     }
//
//     // mps/main-0.mps
//     write!(
//         output,
//         "
// \tbash $(buildroot)/make_pdf.sh main
//
// {pdfname}.pdf : main.pdf
// \tcp main.pdf $@
// "
//     )?;
//
//     for f in &song.lilypondfiles {
//         write!(
//             output,
//             "
// {name}.output/{name}.tex : {name}.ly
// \tbash $(buildroot)/make_lytex.sh {name}
//
// ",
//             name = f
//         )?;
//     }
//
//     for f in &song.lilypondfiles {
//         let f2 = f.replace(".ly", "");
//         write!(
//             output,
//             "
// {f2}.output/{f2}.tex : {f}
// \tbash $(buildroot)/make_lytex.sh {f2}
//
// ",
//             f2 = f2,
//             f = f
//         )?;
//     }
//
//     write!(output, "midi : ")?;
//     for w in &song.wavfiles {
//         let name = w.replace(".wav", "");
//         write!(output, " {name}.midi ", name = name)?;
//     }
//     writeln!(output, "")?;
//
//     for w in &song.wavfiles {
//         let name = w.replace(".wav", "");
//         write!(
//             output,
//             "
// wav : {name}.wav
//
// midi : {name}.midi
//
// {name}.wav {name}.midi : {name}.ly
// \tbash $(buildroot)/make_wav.sh {name}
// ",
//             name = name
//         )?;
//     }
//
//     //
//     // mps/main-0.mps  : main.mp
//     // mkdir -p mps
//     // bash $(buildroot)/make_mpost.sh main.mp
//     //
//     // intro.output/intro.tex : intro.ly
//     // bash $(buildroot)/make_lytex.sh intro
//     //
//     // intro2.output/intro2.tex : intro2.ly
//     // bash $(buildroot)/make_lytex.sh intro2
//     //
//
//     Ok(())
// }

pub fn generate_song_omakefile(song: &Song) -> Result<(), Error> {
    log::debug!("generate Omakefile in {}", song.builddir.display());
    let mut p: PathBuf = song.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakefile");
    let mut output = File::create(p)?;
    let template =
        String::from_utf8(include_bytes!("../../others/makefiles/omakefile").to_vec()).unwrap();

    let mut h = get_handlebar()?;
    h.register_template_string("t1", template).unwrap();
    let output_data = h.render("t1", song).unwrap();
    let _ = output.write(output_data.as_bytes()).unwrap();
    Ok(())
}
