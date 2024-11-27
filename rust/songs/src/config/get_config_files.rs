use std::path::PathBuf;
use std::vec::Vec;
use walkdir::WalkDir;

pub fn get_song_json_paths(current_dir: &PathBuf) -> Vec<PathBuf> {
    let mut filenames: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok()) {
        let x: PathBuf = entry.path().canonicalize().expect("xx").to_path_buf();
        if entry.file_name().to_str() == Some("song.json") {
            filenames.push(x.clone());
        }
    }
    return filenames;
}

pub fn get_book_json_paths(current_dir: &PathBuf) -> Vec<PathBuf> {
    let mut filenames: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok()) {
        let x: PathBuf = entry.path().canonicalize().expect("xx").to_path_buf();
        if entry.file_type().is_file() {
            match entry.file_name().to_str() {
                Some(_) => filenames.push(x.clone()),
                _ => {}
            }
        };
    }
    return filenames;
}
