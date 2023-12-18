use std::path::PathBuf;
use std::vec::Vec;
use walkdir::WalkDir;

pub fn get_song_yml_paths(current_dir: &PathBuf) -> Vec<PathBuf> {
    let mut filenames: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok()) {
        let x: PathBuf = entry.path().canonicalize().expect("xx").to_path_buf();
        if entry.file_name().to_str() == Some("song.yml") {
            filenames.push(x.clone());
        }
    }
    return filenames;
}
