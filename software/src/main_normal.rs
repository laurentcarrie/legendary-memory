use log;
use serde::{Deserialize, Serialize};
// use simple_logger;
use log::LevelFilter;
use simple_logging;

use argh::FromArgs;
use std::path::PathBuf;
use tokio::sync::mpsc;

use tokio::task::JoinSet;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::actions::xxx::build;

// mod app;
// #[cfg(feature = "crossterm")]
// mod crossterm;
// #[cfg(feature = "termion")]
// mod termion;
// mod ui;

pub mod actions;
pub mod generate;
pub mod helpers;
pub mod model;
pub mod ui;

/// cli doc
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "8")]
    nb_workers: u32,
    // #[argh(option, default = "false")]
    // rebuild_all: bool,
    #[argh(positional)]
    songdir: String,
    #[argh(positional)]
    bookdir: String,
    #[argh(positional)]
    builddir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logging::log_to_file("songbook.log", LevelFilter::Info)?;
    //  simple_logging::log_to_stderr(LevelFilter::Info) ;
    // simple_logger::init_with_level(log::Level::Info).unwrap();
    let cli: Cli = argh::from_env();

    build(
        PathBuf::from(cli.songdir),
        PathBuf::from(cli.bookdir),
        PathBuf::from(cli.builddir),
        false,
        cli.nb_workers,
    )
    .await?;
    log::info!("SUCCESS");
    Ok(())
}
