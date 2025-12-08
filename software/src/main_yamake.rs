// use simple_logger;
// use log::LevelFilter;
// use simple_logging;

use argh::FromArgs;
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::path::PathBuf;

pub mod actions;
pub mod generate;
pub mod helpers;
pub mod model;
pub mod ui;
pub mod yamakegraph;

use yamakegraph::graph::make_graph;

use crate::model::world::make;

/// cli doc
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "8")]
    nb_workers: u32,
    /// force rebuild
    #[argh(switch, short = 'f')]
    force: bool,
    #[argh(positional)]
    songdir: String,
    #[argh(positional)]
    bookdir: String,
    #[argh(positional)]
    builddir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let level = log::LevelFilter::Info;
    let file_path = "logs/songbook.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {file}:{line} - {m}\n")))
        .target(Target::Stderr)
        .build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {file}:{line} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("logfile", Box::new(logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                // .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();
    let _handle = log4rs::init_config(config)?;

    log::info!("Starting legendary-memory");

    let cli: Cli = argh::from_env();

    let world = make(
        &PathBuf::from(cli.songdir),
        &PathBuf::from(cli.bookdir),
        &PathBuf::from(cli.builddir),
    )?;

    make_graph(world).await?;

    log::info!("SUCCESS");
    Ok(())
}
