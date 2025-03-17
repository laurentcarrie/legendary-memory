use crate::errors::MyError;
use std::fs;
use std::path::PathBuf;

pub fn create_dir_all(p: &PathBuf) -> Result<(), MyError> {
    match fs::create_dir_all(&p) {
        Ok(()) => Ok(()),
        Err(e) => Err(MyError::MessageError(format!("{:?}, {:?}", &e, &p))),
    }
}

pub fn write(p: &PathBuf, bytes: &[u8]) -> Result<(), MyError> {
    match fs::write(&p, bytes) {
        Ok(()) => Ok(()),
        Err(e) => Err(MyError::MessageError(format!("{:?}, {:?}", &e, &p))),
    }
}
