use band_songbook::{make_all, world_of_srcdir};
use std::path::Path;

fn main() {
    // Initialize logging
    env_logger::init();

    let srcdir = Path::new("tests/data");
    let sandbox = Path::new("sandbox");

    // Create sandbox directory
    std::fs::create_dir_all(sandbox).expect("Failed to create sandbox directory");

    println!("Building all songs...");
    let world = world_of_srcdir(srcdir);
    let (success, _g) = make_all(srcdir, sandbox, None, None, &[], &world);

    if success {
        println!("Build succeeded!");
    } else {
        eprintln!("Build failed!");
        std::process::exit(1);
    }
}
