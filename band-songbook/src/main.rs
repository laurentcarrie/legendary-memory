use argh::FromArgs;
use band_songbook::{StoragePath, make_all_with_storage};
use std::process::ExitCode;

#[derive(FromArgs)]
/// Build all songs in a directory
struct Args {
    /// source directory containing song.yml files (local path or s3://bucket/prefix)
    #[argh(option, short = 's')]
    srcdir: String,

    /// output directory for built files (local path or s3://bucket/prefix)
    #[argh(option, short = 'o')]
    sandbox: String,

    /// path to settings.yml file (local path or s3://bucket/key)
    #[argh(option, short = 'c')]
    settings: Option<String>,

    /// pattern to filter songs (e.g. "black_keys" or "red_hot*")
    #[argh(option, short = 'p')]
    pattern: Option<String>,
}

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    let args: Args = argh::from_env();

    // Validate srcdir exists for local paths
    let srcdir_path = match StoragePath::parse(&args.srcdir) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: invalid srcdir: {e}");
            return ExitCode::from(1);
        }
    };

    if let Some(local_path) = srcdir_path.as_local() {
        if !local_path.exists() {
            eprintln!("Error: srcdir '{}' does not exist", local_path.display());
            return ExitCode::from(1);
        }
    }

    // Validate sandbox path for local paths - create if needed
    let sandbox_path = match StoragePath::parse(&args.sandbox) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: invalid sandbox: {e}");
            return ExitCode::from(1);
        }
    };

    // Determine local sandbox path
    let local_sandbox: std::path::PathBuf;
    let _temp_sandbox: Option<tempfile::TempDir>;

    if let Some(local_path) = sandbox_path.as_local() {
        if let Err(e) = std::fs::create_dir_all(local_path) {
            eprintln!(
                "Error: failed to create sandbox '{}': {}",
                local_path.display(),
                e
            );
            return ExitCode::from(1);
        }
        local_sandbox = local_path.clone();
        _temp_sandbox = None;
    } else {
        // S3 sandbox - create temp directory
        match tempfile::tempdir() {
            Ok(temp) => {
                local_sandbox = temp.path().to_path_buf();
                _temp_sandbox = Some(temp);
            }
            Err(e) => {
                eprintln!("Error: failed to create temp sandbox: {e}");
                return ExitCode::from(1);
            }
        }
    }

    let result: Result<(bool, band_songbook::G), String> = make_all_with_storage(
        &args.srcdir,
        &args.sandbox,
        &local_sandbox,
        args.settings.as_deref(),
        args.pattern.as_deref(),
    )
    .await;

    match result {
        Ok((success, g)) => {
            // Write mermaid graph to sandbox (only for local paths)
            if let Some(local_sandbox) = sandbox_path.as_local() {
                let graph_path = local_sandbox.join("graph.md");
                let graph_content = format!("# graph\n\n```mermaid\n\n{}\n```\n", g.to_mermaid());
                if let Err(e) = std::fs::write(&graph_path, graph_content) {
                    eprintln!("Warning: failed to write graph.md: {e}");
                }
            }

            if success {
                println!("Build succeeded");
                ExitCode::SUCCESS
            } else {
                eprintln!("Build failed");
                ExitCode::from(1)
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::from(1)
        }
    }
}
