use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use log;
use serde::{Deserialize, Serialize};
use simple_logger;

use std::path::PathBuf;

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
    nb_workers: u32,
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
    log::info!("nb_workers : {}", &event.payload.nb_workers);

    let songdir = PathBuf::from(&event.payload.songdir);
    let bookdir = PathBuf::from(&event.payload.bookdir);
    let builddir = PathBuf::from(&event.payload.builddir);
    let nb_workers = event.payload.nb_workers;
    match generate::generate::generate_for_aws_lambda(&PathBuf::from(&builddir)) {
        Ok(()) => (),
        Err(e) => return Err(Error::from(e.to_string())),
    }

    match crate::actions::xxx::build(songdir, bookdir, builddir, true, nb_workers).await {
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
    simple_logger::init_with_level(log::Level::Info)?;

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

    run(service_fn(function_handler)).await?;

    log::info!("SUCCESSß !");
    Ok(())
}
