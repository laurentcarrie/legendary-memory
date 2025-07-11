use color_print::cformat;
use log;
use std::error::Error;

use std::path::PathBuf;
use tokio::sync::mpsc;

use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

use crate::actions::build_pdf::wrapped_build_pdf_book;
use crate::actions::build_pdf::wrapped_build_pdf_song;
use crate::generate::all::generate_all;
use crate::model::model::ELogType;
use crate::model::model::LogItem;
use crate::model::model::LogItemSong;
use crate::model::model::{Book, Song};
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
    let mut books_to_build = world.books.clone();

    let count_songs_to_do = songs_to_build.len();
    let count_books_to_do = books_to_build.len();

    let mut count_songs_done = 0;
    let mut running_songs: Vec<Song> = vec![];
    let mut running_books: Vec<Book> = vec![];
    let mut count_books_done = 0;

    let pb = ProgressBar::new((count_songs_to_do + count_books_to_do) as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
    // let pb = ProgressBar::new_spinner();
    // pb.enable_steady_tick(Duration::from_millis(120));
    // pb.set_style(
    //     ProgressStyle::with_template("{spinner:.blue} {msg}")
    //         .unwrap()
    //         // For more spinners check out the cli-spinners project:
    //         // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
    //         .tick_strings(&[
    //             "▹▹▹▹▹",
    //             "▸▹▹▹▹",
    //             "▹▸▹▹▹",
    //             "▹▹▸▹▹",
    //             "▹▹▹▸▹",
    //             "▹▹▹▹▸",
    //             "▪▪▪▪▪",
    //         ]),
    // );

    loop {
        if (running_songs.len() as u32) < nb_workers {
            if let Some(song) = songs_to_build.pop() {
                log::info!("RUN {} {}", song.author, song.title);
                running_songs.push(song.clone());
                pb.println(cformat!(
                    "<blue>START</blue> song <green>{}</green> @ <cyan>{}</cyan>",
                    song.author,
                    song.title
                ));
                set.spawn(wrapped_build_pdf_song(
                    tx.clone(),
                    world.clone(),
                    song.clone(),
                    force_rebuild,
                ));
            }
        }
        if count_songs_done == count_songs_to_do && (running_books.len() as u32) < nb_workers {
            if let Some(book) = books_to_build.pop() {
                log::info!("BOOK {}", book.title);
                running_books.push(book.clone());
                pb.println(cformat!(
                    "<blue>START</blue> book  <cyan>{}</cyan>",
                    book.title
                ));
                set.spawn(wrapped_build_pdf_book(
                    tx.clone(),
                    world.clone(),
                    book.clone(),
                ));
            }
        }

        if let Some(li) = rx.recv().await {
            log::info!("{}:{} {:?}", file!(), line!(), li);
            match li {
                LogItem::Book(b) => match b.status {
                    ELogType::Success
                    | ELogType::Failed
                    | ELogType::NoNeedFailed
                    | ELogType::NoNeedSuccess => {
                        count_books_done += 1;
                        running_books = running_books
                            .into_iter()
                            .filter(|book| b.title != book.title)
                            .collect::<Vec<_>>();
                        pb.println(cformat!(
                                "<green!>DONE</green!> book <cyan>{}</cyan>",
                                b.title
                            ));                        // pb.set_message(format!("... > book {} ", b.title));

                        pb.inc(1);
                    }
                    ELogType::Started
                    | ELogType::Ps2pdf
                    | ELogType::Lilypond(_)
                    | ELogType::Lualatex(_) => {}
                },
                LogItem::Song(s) => {
                    match s.status {
                        ELogType::Success
                        | ELogType::Failed
                        | ELogType::NoNeedFailed
                        | ELogType::NoNeedSuccess => {
                            count_songs_done += 1;
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
                            // pb.set_message(format!("... > song {} @ {}", s.author, s.title));
                            pb.println(cformat!(
                                "<green!>DONE</green!> song <green>{}</green> @ <cyan>{}</cyan>",
                                s.author,
                                s.title
                            ));

                            pb.inc(1);
                        }
                        ELogType::Lilypond(ly) => {
                            pb.println(format!("lilypond {} @ {} : {}", s.author, s.title, ly));
                        }
                        ELogType::Lualatex(_) | ELogType::Started | ELogType::Ps2pdf => (),
                    }
                }
            }
        }
        if count_songs_done == count_songs_to_do && count_books_done == count_books_to_do {
            break;
        }
    }
    pb.finish_with_message("done");

    Ok(())
}
