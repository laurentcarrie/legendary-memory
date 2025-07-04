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

// //helper function that lists the files all the files in the EFS volume
// async fn list_files() -> Result<String, Error> {
//     let mut files = String::new();
//     for entry in std::fs::read_dir("/mnt/efs")? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             files.push_str(&format!(
//                 "

// {}",
//                 path.display()
//             ));
//         }
//     }
//     Ok(files)
// }

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    log::info!("songdir : {}", &event.payload.songdir);
    log::info!("bookdir : {}", &event.payload.bookdir);
    log::info!("builddir : {}", &event.payload.builddir);

    match generate::all::generate_all(
        PathBuf::from(&event.payload.songdir),
        PathBuf::from(&event.payload.bookdir),
        PathBuf::from(&event.payload.builddir),
    ) {
        Ok(()) => (),
        Err(e) => return Err(Error::from(anyhow::Error::msg(e.to_string()))),
    }
    // .unwrap_or_else(|e| {Err::<_,String>(e.to_string());});

    let world: World = {
        let mut path = PathBuf::from(&event.payload.builddir);
        path.push("world-internal.json");
        let data = std::fs::read_to_string(path.to_str().unwrap()).unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    };

    let (tx, mut rx) = mpsc::channel::<LogItem>(1000);

    for song in world.songs.iter() {
        log::info!("Building PDF for song: {}", song.title);
        let force_rebuild = false; // TODO: make this configurable

        match build_pdf_song(tx.clone(), world.clone(), song.clone(), force_rebuild).await {
            Ok(()) => (),
            Err(e) => {
                log::error!("Error building PDF for song {}: {}", song.title, e);
                return Err(Error::from(anyhow::Error::msg(e.to_string())));
            }
        };
    }
    // let set: JoinSet<()> = JoinSet::new();

    // crossterm::run(world, cli.nb_workers, set, tx, &mut rx, cli._rate).await?;

    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Hello, !"),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // simple_logging::log_to_file("songbook.log", LevelFilter::Info)?;
    simple_logger::init_with_level(log::Level::Info).unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
