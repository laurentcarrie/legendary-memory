use log;
use std::error::Error;
use tui::widgets::canvas::Line;

use tokio::sync::mpsc;

//  #![feature(local_waker)]

use std::path::PathBuf;

/// Demo
// use crate::actions::build_pdf::wrapped_build_pdf;
// #[cfg(feature = "crossterm")]
// use crate::ui::crossterm;
use crate::actions::build_pdf::wrapped_build_pdf_song;
// use crate::generate::generate;
use crate::generate::all::generate_all;
use crate::model::model::ELogType;
use crate::model::model::LogItem;
use crate::model::model::LogItemSong;
use crate::model::model::Song;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::model::model::World;
use std::result::Result;

use std::task::{Context, Poll};

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinSet;

pub async fn watch(mut rx: Receiver<LogItem>) {
    loop {
        log::info!("block on rx");
        let li = rx.recv().await;
        log::info!("{}:{} {:?}", file!(), line!(), li);
    }
}

pub async fn build(
    songdir: PathBuf,
    bookdir: PathBuf,
    builddir: PathBuf,
    force_rebuild: bool,
    nb_workers: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    match generate_all(songdir.clone(), bookdir.clone(), builddir.clone()) {
        Ok(()) => (),
        Err(e) => return Result::Err(e),
    }

    let world: World = {
        let mut path = PathBuf::from(&builddir);
        path.push("world-internal.json");
        let data = std::fs::read_to_string(path.to_str().unwrap()).unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    };

    // log::info!("calling generate_for_aws_lambda");
    // generate::generate::generate_for_aws_lambda(&PathBuf::from(&world.builddir)).unwrap();

    let mut set: JoinSet<()> = JoinSet::new();

    let (tx, mut rx) = mpsc::channel::<LogItem>(1000);

    let mut songs_to_build = world.songs.clone();
    let count_to_do = songs_to_build.len();
    let mut count_done = 0;
    let mut running_songs: Vec<Song> = vec![];

    // set.spawn(watch(rx));

    // for song in &world.songs {
    //     set.spawn(wrapped_build_pdf_song(
    //         tx.clone(),
    //         world.clone(),
    //         song.clone(),
    //         force_rebuild,
    //     ));
    // }

    loop {
        if (running_songs.len() as u32) < nb_workers {
            if let Some(song) = songs_to_build.pop() {
                log::info!("RUN {} {}", song.author, song.title);
                running_songs.push(song.clone());
                set.spawn(wrapped_build_pdf_song(
                    tx.clone(),
                    world.clone(),
                    song.clone(),
                    force_rebuild,
                ));
            }
        }

        if let Some(li) = rx.recv().await {
            log::info!("{}:{} {:?}", file!(), line!(), li);
            match li {
                LogItem::Book(b) => (),
                LogItem::Song(s) => match s.status {
                    ELogType::Success
                    | ELogType::Failed
                    | ELogType::NoNeedFailed
                    | ELogType::NoNeedSuccess => {
                        count_done += 1;
                        let before = running_songs.len();
                        running_songs = running_songs
                            .into_iter()
                            .filter(|song| s.author != song.author || s.title != song.title)
                            .collect::<Vec<_>>();
                        let after = running_songs.len();
                        if after != before - 1 {
                            log::error!("logic error here {:?}", s);
                        }
                        // assert!(count_done + running_songs.len() == count_to_do);
                    }
                    ELogType::Lilypond(_)
                    | ELogType::Lualatex(_)
                    | ELogType::Started
                    | ELogType::Ps2pdf => (),
                },
            }
        }
        if count_done == count_to_do {
            break;
        }
        // if running_songs.is_empty() && songs_to_build.is_empty() {
        //     break;
        // }
    }

    // loop {
    //     'outer: loop {
    //         if running_songs.len() < 4 {
    //             if let Some(s) = songs_to_build.pop() {
    //                 running_songs.push(s.clone());
    //                 let _ = set.spawn(wrapped_build_pdf_song(
    //                     tx.clone(),
    //                     world.clone(),
    //                     s.clone(),
    //                     force_rebuild,
    //                 ));
    //             }
    //         }

    // let preceived = rx.poll_recv(&mut cx);
    // let preceived = rx.();
    // match preceived {
    //     None => {
    //         // let _ = sleep(Duration::from_millis(100)).await;
    //     }
    // break 'outer ;
    // Some(x) => (), // Poll::Pending => {
    //     if !rx.is_empty() {
    //         log::error!("pending but not empty : {}", rx.len());
    //         let _x = sleep(Duration::from_millis(10)).await;
    //     } else {
    //         break 'outer;
    //     }
    // }
    // Poll::Ready(received) => match received {
    //     None => {
    //         log::error!("ready none");
    //         break 'outer;
    //     }
    //     Some(s) => {
    //         log::info!("{}:{} received something {:?}", file!(), line!(), &s);
    //         // break 'outer;
    //     }
    // },
    // }
    // }
    // }
    // for song in world.songs.iter() {
    //     log::info!("Building PDF for song: {}", song.title);
    //     let force_rebuild = false; // TODO: make this configurable

    //     match build_pdf_song(tx.clone(), world.clone(), song.clone(), force_rebuild).await {
    //         Ok(()) => (),
    //         Err(e) => {
    //             log::error!("Error building PDF for song {}: {}", song.title, e);
    //             return Err(e);
    //         }
    //     };
    // }
    // let set: JoinSet<()> = JoinSet::new();

    // crossterm::run(world, cli.nb_workers, set, tx, &mut rx, cli._rate).await?;

    // let resp = Response {
    //     req_id: event.context.request_id,
    //     msg: format!("Hello 222, !"),
    // };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(())
}
