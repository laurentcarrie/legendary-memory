use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};


#![feature(local_waker)]

use std::path::PathBuf;
use std::time::Duration;

use argh::FromArgs;
use log::LevelFilter;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

// use crate::actions::build_pdf::wrapped_build_pdf;
#[cfg(feature = "crossterm")]
use crate::ui::crossterm ;

// use crate::generate::generate::generate;
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
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    files: String,
}

//helper function that lists the files all the files in the EFS volume
async fn list_files() -> Result<String, Error> {
    let mut files = String::new();
    for entry in std::fs::read_dir("/mnt/efs")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push_str(&format!(
                "

{}",
                path.display()
            ));
        }
    }
    Ok(files)
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let cli: Cli = argh::from_env();

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

    crossterm::run(world, cli.nb_workers, set, tx, &mut rx, cli._rate).await?;

    Ok(())

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logging::log_to_file("songbook.log", LevelFilter::Info)?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
