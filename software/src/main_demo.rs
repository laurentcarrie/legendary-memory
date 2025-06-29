#![feature(local_waker)]

use std::path::PathBuf;
use std::time::Duration;

use argh::FromArgs;
use log::LevelFilter;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

// use crate::actions::build_pdf::wrapped_build_pdf;
#[cfg(feature = "crossterm")]
use crate::demo::crossterm::run;
#[cfg(feature = "termion")]
use crate::demo::termion::run;

// use crate::generate::generate::generate;
use crate::model::model::World;

// mod app;
// #[cfg(feature = "crossterm")]
// mod crossterm;
// #[cfg(feature = "termion")]
// mod termion;
// mod ui;

pub mod actions;
pub mod demo;
pub mod generate;
pub mod helpers;
pub mod model;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "8")]
    nb_workers: u32,
    #[argh(option, default = "250", description = "tick rate")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    // #[argh(option, default = "true")]
    // enhanced_graphics: bool,
    #[argh(positional)]
    songdir: String,
    #[argh(positional)]
    bookdir: String,
    #[argh(positional)]
    builddir: String,
}

// async fn watch(mut rx: Receiver<String>) -> () {
//     loop {
//         let received = rx.recv().await;
//         match received {
//             None => println!("huh, NONE !"),
//             Some(s) => {
//                 let mut f = File::options()
//                     .append(true)
//                     .create(true)
//                     .open("date.log")
//                     .unwrap();
//                 writeln!(&mut f, "{}", s).unwrap();
//                 f.flush().unwrap();
//                 // logs.items.insert(0,("blah".to_string(),s)) ;
//             }
//         }
//         // thread::sleep(time::Duration::from_secs(1));
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = simple_logging::log_to_file("songbook.log", LevelFilter::Info)?;
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    match generate::all::generate_all(
        PathBuf::from(&cli.songdir),
        PathBuf::from(&cli.bookdir),
        PathBuf::from(&cli.builddir),
    ) {
        Ok(()) => (),
        Err(e) => {
            log::error!("{}:{} {}", file!(), line!(), e);
            // println!("Custom backtrace: {}", Backtrace::force_capture());
            std::process::exit(1)
        }
    };
    let world: World = {
        let mut path = PathBuf::from(cli.builddir);
        path.push("world-internal.json");
        let data = std::fs::read_to_string(path.to_str().unwrap()).unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    };

    let (tx, mut rx) = mpsc::channel(1000);

    // let mut setwatch: JoinSet<_> = JoinSet::new();
    // setwatch.spawn(watch(rx));
    // setwatch.join_all().await ;

    let set: JoinSet<()> = JoinSet::new();
    // for song in &world.songs {
    //     let _ = set.spawn(wrapped_build_pdf(tx.clone(), song.clone()));
    // }

    run(world, cli.nb_workers, set, tx, &mut rx, tick_rate).await?;

    Ok(())
}
