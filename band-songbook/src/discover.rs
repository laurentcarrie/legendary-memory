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

/// Discovers all book yaml files directly in the given books directory.
/// Returns them sorted by path, so the build graph is stable.
pub fn discover_books(dir: &Path) -> Vec<PathBuf> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut results: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .map(|e| e == "yml" || e == "yaml")
                    .unwrap_or(false)
        })
        .collect();
    results.sort();
    results
}
