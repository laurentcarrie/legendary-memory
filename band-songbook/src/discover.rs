use std::path::{Path, PathBuf};

/// Discovers all `song.yml` files in the subdirectories of the given directory.
pub fn discover(dir: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    discover_recursive(dir, &mut results);
    results
}

fn discover_recursive(dir: &Path, results: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            discover_recursive(&path, results);
        } else if path.file_name().map(|n| n == "song.yml").unwrap_or(false) {
            results.push(path);
        }
    }
}
