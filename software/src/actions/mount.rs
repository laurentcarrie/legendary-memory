use crate::helpers::io::{read_to_string, read_to_vec_u8, write, write_string};
use std::path::PathBuf;

pub fn mount_from_data(bytes: Vec<u8>, target: PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    log::info!("");
    let needs_write = if target.try_exists()? {
        log::info!("{}:{}", file!(), line!());
        let data = read_to_vec_u8(&target)?;
        data != bytes
    } else {
        true
    };
    if needs_write {
        write(&target, &bytes)?
    };
    log::info!("needs write {:?} : {} ", target, needs_write);
    Ok(if needs_write { 1 } else { 0 })
}

pub fn mount_from_file(pfrom: PathBuf, pto: PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    let new_data = read_to_string(&pfrom)?;
    let needs_write = if pto.exists() {
        let old_data = read_to_string(&pto)?;
        old_data != new_data
    } else {
        true
    };
    if needs_write {
        write_string(&pto, &new_data)?;
    };
    Ok(if needs_write { 1 } else { 0 })
}
