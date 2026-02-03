use std::fs::File;
// use tokio::process::Child;
use std::path::PathBuf;
use tokio::process::Command;

use crate::helpers::digest::{digest_has_changed, write_digest};
use crate::model::use_model as M;

pub async fn run(
    _world: M::World,
    song: M::Song,
    wavefile: String,
    deps: Vec<PathBuf>,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut target = song.builddir.clone();
    target.push(&wavefile);
    target.set_extension("midi");

    if target.try_exists()? {
        if !digest_has_changed(target.clone(), deps.clone())? {
            return Ok(M::BuildType::NotTouched(target));
        }
    }

    let mut plyfile = song.builddir.clone();
    plyfile.push(&wavefile);
    plyfile.set_extension("ly");

    if !plyfile.exists() {
        return Err(format!("LilyPond file {} does not exist", plyfile.display()).into());
    }
    let logstream = |s: &str| {
        let mut p: PathBuf = PathBuf::from(&song.builddir);
        p.push(format!("lilypond-midi-{}-{}.log", wavefile, s));
        File::create(p)
    };

    let child = Command::new("lilypond")
        .arg(plyfile.to_str().unwrap())
        .kill_on_drop(true)
        .stdout(logstream("stdout")?)
        .stderr(logstream("stderr")?)
        .current_dir(&song.builddir)
        .spawn()?
        .wait()
        .await?;

    if child.success() {
        write_digest(target.clone(), deps)?;
        Ok(M::BuildType::Rebuilt(target))
    } else {
        Err(format!(
            "lilypond failed for song {} {} ; file {}",
            song.author,
            song.title,
            &plyfile.display()
        )
        .into())
    }
}
