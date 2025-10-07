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

use crate::actions::xxx::build;

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

    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // simple_logging::log_to_file("songbook.log", LevelFilter::Info)?;
    //  simple_logging::log_to_stderr(LevelFilter::Info) ;
    // simple_logger::init_with_level(log::Level::Info).unwrap();

    log::info!("Starting legendary-memory");

    let cli: Cli = argh::from_env();

    let pb = |s, m| {
        let s = PathBuf::from(s);

        if s.is_relative() {
            Err(format!("{m} has to be an absolute path"))
        } else {
            Ok(s)
        }
    };

    build(
        pb(cli.songdir, "songdir")?,
        pb(cli.bookdir, "bookdir")?,
        pb(cli.builddir, "builddir")?,
        cli.force,
        cli.nb_workers,
    )
    .await?;
    log::info!("SUCCESS");
    Ok(())
}
