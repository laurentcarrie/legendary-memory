use log;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use crate::helpers::io::{create_dir_all, read_to_string, read_to_vec_u8, write_string};

fn digest_file_of_target_file(target_file: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut digest_file = PathBuf::from(target_file.parent().ok_or("huh, no parent")?);
    digest_file.push("digest");
    digest_file.push(target_file.file_name().ok_or("huh, no filename")?);
    Ok(digest_file)
}

fn compute_digest(
    target_file: PathBuf,
    deps: Vec<PathBuf>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut digest = Sha256::new();
    let contents = read_to_vec_u8(&&target_file)?;
    digest.update(contents);
    log::info!("start loop");
    for p in deps {
        log::info!("get digest of dep {:?} for {:?}", p, target_file);
        let contents = read_to_vec_u8(&p)?;
        digest.update(contents);
    }
    log::info!("end loop");

    let result = digest.finalize();
    let digest = hex::encode(&result[..]);
    Ok(digest)
}

pub fn write_digest(
    target_file: PathBuf,
    deps: Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let digest_file = digest_file_of_target_file(target_file.clone())?;
    let digest = compute_digest(target_file, deps)?;
    create_dir_all(&PathBuf::from(
        digest_file.parent().ok_or("huh, no parent ?")?,
    ))?;
    write_string(&digest_file.to_path_buf(), &digest)?;
    Ok(())
}

pub fn digest_has_changed(
    target_file: PathBuf,
    deps: Vec<PathBuf>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let digest_file = digest_file_of_target_file(target_file.clone())?;

    let old_digest: Option<String> = {
        if digest_file.exists() {
            Some(read_to_string(&digest_file)?)
        } else {
            None
        }
    };

    let current_digest = compute_digest(target_file.clone(), deps)?;

    let has_changed = {
        if let Some(old_digest) = old_digest {
            old_digest != current_digest
        } else {
            true
        }
    };

    log::info!(
        "{:?} has changed =======> {} ; {:?} ",
        &target_file,
        has_changed,
        digest_file
    );

    Ok(has_changed)
}
