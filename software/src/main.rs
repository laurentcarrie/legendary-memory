use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::fs;
// use std::backtrace::Backtrace;
use std::env;
use std::path::PathBuf;

pub mod actions;
pub mod generate;
pub mod helpers;
pub mod model;

use crate::actions::main_loop::main_loop;

// pub mod protocol ;
fn usage(prog: &str) -> String {
    return format!("usage : {prog} <srcdir> <bookdir> <builddir>", prog = prog);
}

#[tokio::main]
async fn main() -> () {
    SimpleLogger::new().init().unwrap();
    // log::set_max_level(LevelFilter::Debug);
    log::set_max_level(LevelFilter::Info);
    log::info!("start songbook");
    let args: Vec<String> = env::args().collect();
    let (songdir, bookdir, builddir) = match (args.get(1), args.get(2), args.get(3)) {
        (Some(x), Some(y), Some(z)) => (x, y, z),
        _ => {
            panic!("{}", usage(&args[0]));
        }
    };

    match generate::all::generate_all(
        PathBuf::from(songdir),
        PathBuf::from(bookdir),
        PathBuf::from(builddir),
    ) {
        Ok(()) => (),
        Err(e) => {
            log::error!("{}:{} {}", file!(), line!(), e);
            // println!("Custom backtrace: {}", Backtrace::force_capture());
            std::process::exit(1)
        }
    };
    let world = {
        let mut path = PathBuf::from(builddir);
        path.push("world-internal.json");
        let data = fs::read_to_string(path.to_str().unwrap()).unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    };
    log::info!("normal exit");
    let _x = main_loop(&world).await;
    ()
}
