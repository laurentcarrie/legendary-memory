use crate::model::use_model::Song;
use std::process::{ExitStatus, Stdio};
// use tokio::process::Child;
use tokio::process::Command;

pub async fn build_wav(song: Song, wavefile: String) -> Result<(), Box<dyn std::error::Error>> {
    log::info!(
        "Skipped ; Starting wav {} for {} {}",
        wavefile,
        song.author,
        song.title
    );
    Ok(())
}

pub async fn build_wav2(song: Song, wavefile: String) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("begin wav {} for {} {}", wavefile, song.author, song.title);

    let mut plyfile = song.builddir.clone();
    plyfile.push(&wavefile);
    plyfile.set_extension("ly");

    if !plyfile.exists() {
        return Err(format!("LilyPond file {} does not exist", plyfile.display()).into());
    }

    let mut status = ExitStatus::default();

    let child = Command::new("lilypond")
        .arg(plyfile.to_str().unwrap())
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(&song.builddir)
        .spawn();
    match child {
        Ok(mut child) => {
            status = child.wait().await?;
        }
        Err(e) => {
            log::error!(
                "lilypond for {} {}/{}",
                &plyfile.display(),
                song.author,
                song.title
            );
            return Err(e.into());
        }
    }

    if !status.success() {
        return Err(format!(
            "lilypond failed for song {} {} ; file {}",
            song.author,
            song.title,
            &plyfile.display()
        )
        .into());
    }

    let mut pmidifile = song.builddir.clone();
    pmidifile.push(&wavefile);
    pmidifile.set_extension("midi");

    if !pmidifile.exists() {
        return Err(format!("MIDI file {} does not exist", pmidifile.display()).into());
    }
    // rust/songs/others/shfiles/make_wav.sh:fluidsynth --gain 4 -F $1.wav /usr/share/sounds/sf2/FluidR3_GM.sf2  $1.midi 1>>$1.wav.stdout 2>>$1.wav.stderr || true

    log::debug!("found midi file {}", pmidifile.display());

    let mut status = ExitStatus::default();
    let child = Command::new("fluidsynth")
        .arg("--gain")
        .arg("4")
        .arg("-F")
        .arg(wavefile.as_str())
        .arg("/usr/share/sounds/sf2/FluidR3_GM.sf2")
        .arg(pmidifile.to_str().ok_or("Invalid MIDI file path")?)
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(&song.builddir)
        .spawn();
    log::info!("{child:?}");
    match child {
        Ok(mut child) => {
            status = child.wait().await?;
            log::debug!(
                "lilypond {} for {} {}, status {:?}",
                wavefile,
                song.author,
                song.title,
                &status
            );
            // Ok(child) ;
        }
        Err(_e) => {
            log::error!("wav for {} {}/{}", &wavefile, song.author, song.title);
            // Err(MyError::MessageError(format!(
            //     "song : {}/{}, building tex for {}, error id {:?}",
            //     &song.author, &song.title, lyfile, e
            // )))
        }
    }
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "fluidsynth failed for song {} {} ; file {}",
            song.author, song.title, &wavefile
        )
        .into())
    }
}
