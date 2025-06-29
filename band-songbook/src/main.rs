use argh::FromArgs;
use band_songbook::make_all;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(FromArgs)]
/// Build all songs in a directory
struct Args {
    /// source directory containing song.yml files
    #[argh(positional)]
    srcdir: PathBuf,

    /// output directory for built files
    #[argh(positional)]
    sandbox: PathBuf,
}

fn main() -> ExitCode {
    env_logger::init();
    let args: Args = argh::from_env();

    if !args.srcdir.exists() {
        eprintln!("Error: srcdir '{}' does not exist", args.srcdir.display());
        return ExitCode::from(1);
    }

    if let Err(e) = std::fs::create_dir_all(&args.sandbox) {
        eprintln!("Error: failed to create sandbox '{}': {}", args.sandbox.display(), e);
        return ExitCode::from(1);
    }

    let (success, g) = make_all(&args.srcdir, &args.sandbox);

    // Write mermaid graph to sandbox
    let graph_path = args.sandbox.join("graph.md");
    let graph_content = format!("# graph\n\n```mermaid\n\n{}\n```\n", g.to_mermaid());
    if let Err(e) = std::fs::write(&graph_path, graph_content) {
        eprintln!("Warning: failed to write graph.md: {}", e);
    }

    if success {
        println!("Build succeeded");
        ExitCode::SUCCESS
    } else {
        eprintln!("Build failed");
        ExitCode::from(1)
    }
}
