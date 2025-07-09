use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use log;
use serde::{Deserialize, Serialize};
use simple_logger;
use tokio::sync::mpsc;

// #![feature(local_waker)]

use std::path::PathBuf;
use std::time::Duration;

use argh::FromArgs;
use log::LevelFilter;
use tokio::task::JoinSet;

/// Demo
// use crate::actions::build_pdf::wrapped_build_pdf;
// #[cfg(feature = "crossterm")]
// use crate::ui::crossterm;
use crate::actions::build_pdf::build_pdf_song;
// use crate::generate::generate::generate;
use crate::model::model::LogItem;
use crate::model::model::World;

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

#[derive(Deserialize)]
struct Request {
    songdir: String,
    bookdir: String,
    builddir: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    log::info!("songdir : {}", &event.payload.songdir);
    log::info!("bookdir : {}", &event.payload.bookdir);
    log::info!("builddir : {}", &event.payload.builddir);

    let songdir = PathBuf::from(&event.payload.songdir);
    let bookdir = PathBuf::from(&event.payload.bookdir);
    let builddir = PathBuf::from(&event.payload.builddir);

    match crate::actions::xxx::build(songdir, bookdir, builddir,true).await {
        Ok(()) => {
            let resp = Response {
                req_id: event.context.request_id,
                msg: format!("Hello 222, !"),
            };
            Ok(resp)
        }
        Err(e) => Err(Error::from(e.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // simple_logging::log_to_file("songbook.log", LevelFilter::Info)?;
    simple_logger::init_with_level(log::Level::Info).unwrap();
    log::info!("Starting songbook lambda...");
    log::info!(
        "current dir: {}",
        std::env::current_dir().unwrap().display()
    );
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
