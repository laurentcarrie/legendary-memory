use std::fs::File;
// use tokio::process::Child;
use crate::helpers::digest::{digest_has_changed, write_digest};
use std::path::PathBuf;
use tokio::process::Command;

use crate::model::use_model as M;

pub async fn run(
    _world: M::World,
    song: M::Song,
    wavefile: String,
    deps: Vec<PathBuf>,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut target = song.builddir.clone();
    target.push(&wavefile);
    let mut pmidifile = song.builddir.clone();
    pmidifile.push(&wavefile);
    pmidifile.set_extension("midi");

    if target.try_exists()? {
        if !digest_has_changed(target.clone(), deps.clone())? {
            return Ok(M::BuildType::NotTouched(target));
        }
    }

    if !pmidifile.exists() {
        return Err(format!("MIDI file {} does not exist", pmidifile.display()).into());
    }
    // rust/songs/others/shfiles/make_wav.sh:fluidsynth --gain 4 -F $1.wav /usr/share/sounds/sf2/FluidR3_GM.sf2  $1.midi 1>>$1.wav.stdout 2>>$1.wav.stderr || true

    log::debug!("found midi file {}", pmidifile.display());

    let logstream = |s: &str| {
        let mut p: PathBuf = PathBuf::from(&song.builddir);
        p.push(format!("fluidsynth-{}-{}.log", wavefile, s));
        File::create(p)
    };

    let child = Command::new("fluidsynth")
        .arg("--gain")
        .arg("4")
        .arg("-F")
        .arg(wavefile.as_str())
        .arg("/usr/share/sounds/sf2/FluidR3_GM.sf2")
        .arg(pmidifile.to_str().ok_or("Invalid MIDI file path")?)
        .kill_on_drop(true)
        .stdout(logstream("stdout")?)
        .stderr(logstream("stderr")?)
        .current_dir(&song.builddir)
        .spawn()?
        .wait()
        .await?;

    if child.success() {
        write_digest(target.clone(), deps.clone())?;
        Ok(M::BuildType::Rebuilt(target))
    } else {
        log::error!("wav for {} {}/{}", &wavefile, song.author, song.title);
        Err(format!(
            "song : {}/{}, building tex for {}, error id {:?}",
            &song.author,
            &song.title,
            wavefile,
            child.code()
        )
        .into())
    }
}
