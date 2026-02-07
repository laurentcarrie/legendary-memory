pub mod chords;
pub mod discover;
pub mod helpers;
pub mod model;
pub mod nodes;
pub mod settings;
pub mod storage;

pub use discover::discover;
pub use nodes::PdfFile;
pub use storage::StoragePath;

use model::{SectionItem, Song};
use nodes::{PdfCopyFile, SongYml, TexFile};
use object_store::ObjectStoreExt;
use std::path::Path;

pub use yamake::model::G;
use yamake::model::GNode;

/// Returns all LilyPond files (.ly) referenced by a PdfFile node.
/// Scans the tex files to find \lyfile{}, \songly{}, and \include directives.
pub fn get_lilypond_files(
    pdf: &PdfFile,
    sandbox: &Path,
    predecessors: &[&(dyn GNode + Send + Sync)],
) -> Vec<std::path::PathBuf> {
    let (_, inputs, _) = pdf.scan_with_toplevel_ly(sandbox, predecessors);
    inputs
        .into_iter()
        .filter(|p| p.extension().map(|e| e == "ly").unwrap_or(false))
        .collect()
}

/// Checks if pattern is a subsequence of text (fuzzy match).
/// Each character in pattern must appear in text in order, but not necessarily contiguously.
fn fuzzy_match(text: &str, pattern: &str) -> bool {
    let mut text_chars = text.chars().peekable();
    for pat_char in pattern.chars() {
        loop {
            match text_chars.next() {
                Some(c) if c == pat_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

/// Discovers all songs in the given directory and builds them all.
/// Returns (success, graph) where success is true if all builds succeeded.
/// If settings_path is provided, it will be copied to sandbox/settings.yml.
/// If pattern is provided, only songs matching the pattern will be built.
pub fn make_all(
    srcdir: &Path,
    sandbox: &Path,
    settings_path: Option<&Path>,
    pattern: Option<&str>,
) -> (bool, G) {
    let songs = discover(srcdir);
    let songs_sandbox = sandbox.join("songs");
    let mut g = G::new(srcdir.to_path_buf(), songs_sandbox.clone());

    // Check that lualatex is available
    if std::process::Command::new("lualatex")
        .arg("--help")
        .output()
        .is_err()
    {
        log::error!("lualatex is not available. Please install TeX Live or similar.");
        return (false, g);
    }

    // Create songs directory
    if let Err(e) = std::fs::create_dir_all(&songs_sandbox) {
        log::error!("Failed to create songs directory: {e}");
    }

    // Copy settings.yml to songs sandbox if provided
    if let Some(settings) = settings_path {
        let dest = songs_sandbox.join("settings.yml");
        if let Err(e) = std::fs::copy(settings, &dest) {
            log::error!("Failed to copy settings.yml to sandbox: {e}");
        }
    }

    if songs.is_empty() {
        return (true, g);
    }

    // Create pdf directory for copied PDFs
    let pdf_dir = sandbox.join("pdf");
    if let Err(e) = std::fs::create_dir_all(&pdf_dir) {
        log::error!("Failed to create pdf directory: {e}");
    }

    // Create pdf-lyrics directory for copied lyrics PDFs
    let pdf_lyrics_dir = sandbox.join("pdf-lyrics");
    if let Err(e) = std::fs::create_dir_all(&pdf_lyrics_dir) {
        log::error!("Failed to create pdf-lyrics directory: {e}");
    }

    for song_path in songs {
        // Convert absolute path to relative path from srcdir
        let rel_path = match song_path.strip_prefix(srcdir) {
            Ok(p) => p.to_path_buf(),
            Err(_) => continue,
        };

        let parent_dir = rel_path.parent().unwrap_or(Path::new(""));

        // Read song.yml to get structure for lyrics files
        let song: Option<Song> = std::fs::read_to_string(&song_path)
            .ok()
            .and_then(|content| serde_yaml::from_str(&content).ok());

        // Filter by pattern if provided (fuzzy match against author + title)
        if let Some(pat) = pattern {
            let matches = song.as_ref().is_some_and(|s| {
                let search_str = format!("{} {}", s.info.author, s.info.title).to_lowercase();
                fuzzy_match(&search_str, &pat.to_lowercase())
            });
            if !matches {
                continue;
            }
        }

        // Create SongYml node
        let song_node = SongYml::new(rel_path.clone());
        let song_idx = match g.add_root_node(song_node) {
            Ok(idx) => idx,
            Err(_) => continue,
        };

        // Add body.tex as root node (source file from srcdir, not generated)
        let body_path = parent_dir.join("body.tex");
        let body_node = TexFile::new(body_path);
        let _ = g.add_root_node(body_node);

        // Add lyrics files as root nodes for each Chords and Ref section
        if let Some(ref song_data) = song {
            for item in &song_data.structure {
                match &item.item {
                    SectionItem::Chords(_) | SectionItem::Ref(_) => {
                        let lyrics_path =
                            parent_dir.join("lyrics").join(format!("{}.tex", item.id));
                        let lyrics_node = TexFile::new(lyrics_path);
                        let _ = g.add_root_node(lyrics_node);
                    }
                    _ => {}
                }
            }
        }

        // Create matching PdfFile node (main.pdf in same directory)
        let pdf_path = parent_dir.join("main.pdf");
        let pdf_node = PdfFile::new(pdf_path.clone());
        let pdf_idx = match g.add_node(pdf_node) {
            Ok(idx) => idx,
            Err(_) => continue,
        };

        g.add_edge(song_idx, pdf_idx);

        // Create PdfCopyFile node to copy the PDF to ../pdf/<author>--@--<title>.pdf
        if let Some(ref song_data) = song {
            let pdf_copy_path =
                Path::new("../pdf").join(format!("{}.pdf", song_data.info.pdf_name_of_song()));
            let pdf_copy_node = PdfCopyFile::new(pdf_copy_path);
            if let Ok(pdf_copy_idx) = g.add_node(pdf_copy_node) {
                g.add_edge(pdf_idx, pdf_copy_idx);
            }

            // Create lyrics PdfFile node
            let lyrics_pdf_path = parent_dir.join("lyrics").join("main.pdf");
            let lyrics_pdf_node = PdfFile::new(lyrics_pdf_path.clone());
            if let Ok(lyrics_pdf_idx) = g.add_node(lyrics_pdf_node) {
                g.add_edge(song_idx, lyrics_pdf_idx);

                // Create PdfCopyFile node for lyrics PDF
                let lyrics_copy_path = Path::new("../pdf-lyrics")
                    .join(format!("{}-lyrics.pdf", song_data.info.pdf_name_of_song()));
                let lyrics_copy_node = PdfCopyFile::new(lyrics_copy_path);
                if let Ok(lyrics_copy_idx) = g.add_node(lyrics_copy_node) {
                    g.add_edge(lyrics_pdf_idx, lyrics_copy_idx);
                }
            }
        }
    }

    let success = g.make();

    // Move make-report.yml from songs_sandbox to sandbox root
    let report_src = songs_sandbox.join("make-report.yml");
    let report_dest = sandbox.join("make-report.yml");
    if report_src.exists() {
        if let Err(e) = std::fs::rename(&report_src, &report_dest) {
            log::error!("Failed to move make-report.yml: {e}");
        }
    }

    // Move logs directory from songs_sandbox to sandbox root
    let logs_src = songs_sandbox.join("logs");
    let logs_dest = sandbox.join("logs");
    if logs_dest.exists() {
        let _ = std::fs::remove_dir_all(&logs_dest);
    }
    if logs_src.exists() {
        if let Err(e) = std::fs::rename(&logs_src, &logs_dest) {
            log::error!("Failed to move logs directory: {e}");
        }
    }

    (success, g)
}

/// Async version of make_all that supports S3 paths.
///
/// This function:
/// - Parses paths to determine if S3 or local
/// - Downloads S3 srcdir to temp directory if needed
/// - Calls existing `make_all()` with local paths
/// - Uploads sandbox to S3 if sandbox was S3 URL
/// - Cleans up temp directory
///
/// The `local_sandbox` parameter specifies where to run the build locally.
/// If `sandbox` is an S3 path, results will be uploaded there after the build.
pub async fn make_all_with_storage(
    srcdir: &str,
    sandbox: &str,
    local_sandbox: &Path,
    settings: Option<&str>,
    pattern: Option<&str>,
) -> Result<(bool, G), String> {
    use storage::{StoragePath, download_to_local};

    let srcdir_path = StoragePath::parse(srcdir)?;
    let sandbox_path = StoragePath::parse(sandbox)?;
    let settings_path = settings.map(StoragePath::parse).transpose()?;

    // Determine local paths for the build
    // This temp variable holds TempDir handle to keep directory alive until end of function
    let _temp_srcdir: Option<tempfile::TempDir>;

    let local_srcdir: std::path::PathBuf;

    // Handle srcdir
    if srcdir_path.is_s3() {
        let temp = tempfile::tempdir().map_err(|e| format!("Failed to create temp srcdir: {e}"))?;
        local_srcdir = temp.path().to_path_buf();
        _temp_srcdir = Some(temp);
        log::info!("Downloading srcdir from {srcdir} to {local_srcdir:?}");
        download_to_local(&srcdir_path, &local_srcdir).await?;
    } else {
        _temp_srcdir = None;
        local_srcdir = srcdir_path.as_local().unwrap().clone();
    }

    // Create local sandbox if it doesn't exist
    std::fs::create_dir_all(local_sandbox).map_err(|e| format!("Failed to create sandbox: {e}"))?;

    // Handle settings - download if S3, otherwise use local path
    let local_settings: Option<std::path::PathBuf>;
    let _temp_settings: Option<tempfile::NamedTempFile>;

    if let Some(ref sp) = settings_path {
        if sp.is_s3() {
            // Download settings file to a temp file
            let temp_file = tempfile::NamedTempFile::new()
                .map_err(|e| format!("Failed to create temp settings file: {e}"))?;
            let temp_path = temp_file.path().to_path_buf();

            // Download settings from S3
            let (bucket, prefix) = match sp {
                StoragePath::S3 { bucket, prefix } => (bucket, prefix),
                _ => unreachable!(),
            };
            let store = storage::create_s3_client(bucket).await?;

            let object_path = object_store::path::Path::from(prefix.as_str());
            let data = store
                .get(&object_path)
                .await
                .map_err(|e| format!("Failed to get settings from S3: {e}"))?;
            let bytes = data
                .bytes()
                .await
                .map_err(|e| format!("Failed to read settings from S3: {e}"))?;
            std::fs::write(&temp_path, &bytes)
                .map_err(|e| format!("Failed to write temp settings: {e}"))?;

            local_settings = Some(temp_path);
            _temp_settings = Some(temp_file);
        } else {
            local_settings = Some(sp.as_local().unwrap().clone());
            _temp_settings = None;
        }
    } else {
        local_settings = None;
        _temp_settings = None;
    }

    // Run the build
    let (success, g) = make_all(
        &local_srcdir,
        local_sandbox,
        local_settings.as_deref(),
        pattern,
    );

    // Upload sandbox to S3 if needed
    if sandbox_path.is_s3() {
        log::info!("Uploading sandbox to {sandbox}");

        // Collect paths from PdfFile and PdfCopyFile nodes and make-report.yml
        let mut paths_to_upload: Vec<std::path::PathBuf> =
            g.g.node_weights()
                .filter(|node| node.tag() == "pdf" || node.tag() == "pdfcopy")
                .map(|node| node.pathbuf())
                .collect();

        // Also upload make-report.yml if it exists
        let report_path = local_sandbox.join("make-report.yml");
        if report_path.exists() {
            paths_to_upload.push(report_path);
        } else {
            log::warn!("make-report.yml not found at {}", report_path.display());
        }

        // Upload all log files (stdout/stderr) from the logs directory recursively
        let logs_dir = local_sandbox.join("logs");
        if logs_dir.exists() {
            fn collect_files_recursive(dir: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            files.push(path);
                        } else if path.is_dir() {
                            collect_files_recursive(&path, files);
                        }
                    }
                }
            }
            collect_files_recursive(&logs_dir, &mut paths_to_upload);
        }

        log::info!("Uploading {} files to S3", paths_to_upload.len());
        storage::upload_paths_to_s3(&paths_to_upload, local_sandbox, &sandbox_path).await?;
    }

    // Temp directories are automatically cleaned up when dropped
    Ok((success, g))
}

#[cfg(test)]
mod tests;
