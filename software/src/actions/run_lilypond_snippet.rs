use std::fs::File;
use std::io::prelude::*;
// use std::process::{ExitStatus, Stdio};
// use tokio::process::Child;
use crate::helpers::digest::{digest_has_changed, write_digest};
// use crate::helpers::path::make_path;
use std::path::PathBuf;
use tokio::process::Command;

use crate::model::use_model as M;

pub async fn run(
    _world: M::World,
    song: M::Song,
    lyfile: String,
    deps: Vec<PathBuf>,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut target = song.builddir.clone();
    target.push(&lyfile);
    target.set_extension("output");
    target.push(&lyfile);
    target.set_extension("tex");

    let mut plyfiletex = song.builddir.clone();
    plyfiletex.push(&lyfile);
    plyfiletex.set_extension("lytex");
    let mut file = File::create(plyfiletex.to_str().unwrap())?;
    file.write_all(format!("\\lilypondfile{{{}}} ", lyfile.as_str()).as_bytes())?;

    let mut plyfiletex_noextension = plyfiletex.clone();
    plyfiletex_noextension.set_extension("");

    let mut outputdir = song.builddir.clone();
    outputdir.push(format!(
        "{}.output",
        plyfiletex
            .with_extension("")
            .file_name()
            .ok_or("huh ?")?
            .to_str()
            .ok_or("huh ?")?
    ));

    if target.try_exists()? {
        if !digest_has_changed(target.clone(), deps.clone())? {
            return Ok(M::BuildType::NotTouched(target));
        }
    }

    for _i in 0..2 {
        let fout = File::create(PathBuf::from("lilypond.stdout"))?;
        let ferr = File::create(PathBuf::from("lilypond.stderr"))?;

        let child = Command::new("lilypond-book")
            .arg("--output")
            // .arg(format!("{}.output", plyfiletex_noextension.to_str().unwrap()).as_str())
            .arg(outputdir.to_str().unwrap())
            .arg("--pdf")
            .arg("--latex-program=lualatex")
            .arg(plyfiletex.to_str().unwrap())
            .kill_on_drop(true)
            .stdout(fout)
            .stderr(ferr)
            .current_dir(&song.builddir)
            .spawn();
        log::info!("{:?}", child);
        match child {
            Ok(mut child) => {
                let status = child.wait().await?;
                log::debug!(
                    "lilypond {} for {} {}, status {:?}",
                    lyfile,
                    song.author,
                    song.title,
                    &status
                );
                // Ok(child) ;
            }
            Err(_e) => {
                log::error!("lilypond for {} {}/{}", &lyfile, song.author, song.title);
                return Err(format!("lilypond-book for {}", lyfile).into());
            }
        }
    }
    write_digest(target.clone(), deps)?;
    Ok(M::BuildType::Rebuilt(target))
}
