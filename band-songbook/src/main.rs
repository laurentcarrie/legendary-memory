use argh::FromArgs;
use band_songbook::make_all_with_storage;
use std::process::ExitCode;

#[derive(FromArgs)]
/// Build all songs in a directory
struct Args {
    /// source directory containing song.yml files (local path or s3://bucket/prefix)
    #[argh(option, short = 's')]
    songs_srcdir: String,

    /// source directory containing the book yaml files (local path or s3://bucket/prefix)
    #[argh(option, short = 'b')]
    books_srcdir: Option<String>,

    /// local output directory for built files
    #[argh(option, short = 'o')]
    sandbox: String,

    /// path to settings.yml file (local path or s3://bucket/key)
    #[argh(option, short = 'c')]
    settings: String,

    /// fuzzy filter, matched as a subsequence against "<author> <title>" from
    /// song.yml (not the directory name), case-insensitive but accent-sensitive:
    /// "black keys" and "finist" match, "black_keys" and "finistere" do not
    #[argh(option, short = 'p')]
    pattern: Option<String>,

    /// delivery directory for final files (local path or s3://bucket/prefix)
    #[argh(option, short = 'd')]
    delivery: String,

    /// directory containing drum pattern YAML files (can be repeated)
    #[argh(option)]
    drum_patterns_dir: Vec<String>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> ExitCode {
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        println!("band-songbook {VERSION}");
        return ExitCode::SUCCESS;
    }

    env_logger::init();
    let args: Args = argh::from_env();

    // Create sandbox directory
    let sandbox = std::path::PathBuf::from(&args.sandbox);
    if let Err(e) = std::fs::create_dir_all(&sandbox) {
        eprintln!(
            "Error: failed to create sandbox '{}': {}",
            sandbox.display(),
            e
        );
        return ExitCode::from(1);
    }

    let drum_patterns_dirs: Vec<std::path::PathBuf> = args
        .drum_patterns_dir
        .iter()
        .map(std::path::PathBuf::from)
        .collect();
    let result: Result<(bool, band_songbook::G), String> = make_all_with_storage(
        &args.songs_srcdir,
        args.books_srcdir.as_deref(),
        &sandbox,
        Some(args.settings.as_str()),
        args.pattern.as_deref(),
        &args.delivery,
        &drum_patterns_dirs,
    )
    .await;

    match result {
        Ok((success, g)) => {
            // Write mermaid graph to sandbox
            let graph_path = sandbox.join("graph.md");
            let graph_content = format!("# graph\n\n```mermaid\n\n{}\n```\n", g.to_mermaid());
            if let Err(e) = std::fs::write(&graph_path, graph_content) {
                eprintln!("Warning: failed to write graph.md: {e}");
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
