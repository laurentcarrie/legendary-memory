use crate::helpers::digest::{digest_has_changed, write_digest};
use crate::helpers::io::copy_file;
use crate::model::use_model as M;
use std::path::PathBuf;

pub async fn run(
    world: M::World,
    song: M::Song,
    deps: Vec<PathBuf>,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut pfrom: PathBuf = PathBuf::from(&song.builddir);
    pfrom.push("main.pdf");
    let mut target: PathBuf = PathBuf::from(&world.builddir);
    target.push(&"delivery");
    std::fs::create_dir_all(&target)?;
    target.push(song.pdfname);
    target.set_extension("pdf");

    if target.try_exists()? {
        if !digest_has_changed(target.clone(), deps.clone())? {
            return Ok(M::BuildType::NotTouched(target));
        }
    }

    copy_file(&pfrom, &target)?;
    write_digest(target.clone(), deps)?;

    Ok(M::BuildType::Rebuilt(target))
}
