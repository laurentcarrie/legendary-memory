use crate::model::use_model::Song;
use std::fs::File;
use std::io::prelude::*;
use std::process::{ExitStatus, Stdio};
// use tokio::process::Child;
use tokio::process::Command;

pub async fn build_lytex(song: Song, lyfile: String) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!(
        "begin lilypond {} for {} {}",
        lyfile,
        song.author,
        song.title
    );

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
            .ok_or("XXX")?
            .to_str()
            .unwrap()
    ));
    log::debug!(
        "{}:{} lytexfile : {}",
        file!(),
        line!(),
        &plyfiletex.display()
    );
    let mut status = ExitStatus::default();
    for _index in 0..2 {
        let child = Command::new("lilypond-book")
            .arg("--output")
            // .arg(format!("{}.output", plyfiletex_noextension.to_str().unwrap()).as_str())
            .arg(outputdir.to_str().unwrap())
            .arg("--pdf")
            .arg("--latex-program=lualatex")
            .arg(plyfiletex.to_str().unwrap())
            .kill_on_drop(true)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&song.builddir)
            .spawn();
        match child {
            Ok(mut child) => {
                status = child.wait().await?;
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
                // Err(MyError::MessageError(format!(
                //     "song : {}/{}, building tex for {}, error id {:?}",
                //     &song.author, &song.title, lyfile, e
                // )))
            }
        }
    }
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "lilypond failed for song {} {} ; file {}",
            song.author, song.title, &lyfile
        )
        .into())
    }
}
