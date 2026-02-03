use crate::helpers::digest::{digest_has_changed, write_digest};
use crate::helpers::io::copy_file;
use crate::helpers::path::make_path;
use crate::model::use_model as M;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub async fn run(
    _world: M::World,
    song: M::Song,
    filename: String,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let pfrom = make_path(PathBuf::from(song.srcdir.clone()), vec![filename.as_str()]);
    let pto = make_path(song.builddir.clone(), vec![filename.as_str()]);

    create_dir_all(pto.parent().ok_or("huh")?)?;

    if pto.try_exists()? {
        if !digest_has_changed(pto.clone(), vec![])? {
            return Ok(M::BuildType::NotTouched(pto));
        }
    }
    copy_file(&pfrom, &pto)?;
    write_digest(pto.clone(), vec![])?;
    Ok(M::BuildType::Rebuilt(pto))
}
