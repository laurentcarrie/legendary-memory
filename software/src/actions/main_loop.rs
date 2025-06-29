use human_sort::compare;
use std::cmp::Ordering;
// use std::sync::{Arc, Mutex};
// use std::time::{Duration, Instant};
// use std::{fs, thread, time};
use tokio::task::JoinSet;
// use usize_cast;
// use usize_cast::{FromIsize, FromUsize, IntoIsize, IntoUsize};
//use usize_cast::FromUsize;

use crate::actions::build_pdf::{wrapped_build_pdf_book, wrapped_build_pdf_song};
use crate::model::model::World;
use console::{style, Emoji};
// use indicatif::{ MultiProgress, ProgressBar, ProgressStyle};
use tokio::sync::mpsc;

// use crossterm::style::style;

// static PACKAGES: &[&str] = &[
//     "fs-events",
//     "my-awesome-module",
//     "emoji-speaker",
//     "wrap-ansi",
//     "stream-browserify",
//     "acorn-dynamic-import",
// ];

// static COMMANDS: &[&str] = &[
//     "cmake .",
//     "make",
//     "make clean",
//     "gcc foo.c -o foo",
//     "gcc bar.c -o bar",
//     "./helper.sh rebuild-cache",
//     "make all-clean",
//     "make test",
// ];

// static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
// static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
// static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
// static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub async fn main_loop(world: &World) -> () {
    // let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
    //     .unwrap()
    //     .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    // println!(
    //     "{} {}Read sql file...",
    //     style("[1/4]").bold().dim(),
    //     LOOKING_GLASS
    // );
    // let before: DateTime<Utc> = Utc::now();
    // println!(
    //     "{} {}Fetching packages...",
    //     style("[2/4]").bold().dim(),
    //     TRUCK
    // );
    //
    // println!(
    //     "{} {}Linking dependencies...",
    //     style("[3/4]").bold().dim(),
    //     CLIP
    // );
    // let deps = 1232;
    // let pb = ProgressBar::new(deps);
    // for _ in 0..deps {
    //     thread::sleep(Duration::from_millis(3));
    //     pb.inc(1);
    // }
    // pb.finish_and_clear();

    println!(
        "{} {}Building fresh packages...",
        style("[4/4]").bold().dim(),
        PAPER
    );
    // let m = MultiProgress::new();
    let mut songs = world.songs.clone();
    songs.sort_by(|a, b| match compare(a.author.as_str(), b.author.as_str()) {
        Ordering::Equal => compare(a.title.as_str(), b.title.as_str()),
        x => x,
    });

    let mut books = world.books.clone();
    books.sort_by(|a, b| compare(a.title.as_str(), b.title.as_str()));

    // let mmm = Mutex::new(5);

    // {
    //     let counter = Arc::new(Mutex::new(0));
    //     let mut handles = vec![];
    //
    //     for _ in 0..10 {
    //         let counter = Arc::clone(&counter);
    //         let handle = thread::spawn(move || {
    //             let mut num = counter.lock().unwrap();
    //
    //             *num += 1;
    //         });
    //         handles.push(handle);
    //     }
    //
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    //
    //     println!("Result: {}", *counter.lock().unwrap());
    // }

    // let handles: Vec<_> = songs
    //     .into_iter()
    //     .map(|song| {
    //         let world = world.clone();
    //         let pb = m.add(ProgressBar::new(100));
    //         pb.set_style(spinner_style.clone());
    //         pb.set_prefix(format!("[{}][{}]", &song.author, &song.title));
    //         // thread::spawn_blocking( move || {
    //             tokio::task::spawn_blocking(move || {
    //             pb.set_message(format!("running..."));
    //             let x =build_pdf(&world.builddir, &song.builddir) ;
    //             let mut rng = rand::rng();
    //             thread::sleep(Duration::from_millis(rng.random_range(10_000..30_000)));
    //             // thread::sleep(Duration::from_millis(10_000));
    //             pb.finish_with_message("DONE.");
    //         })
    //     })
    //     .collect();

    // let count = 0;
    // loop {
    //     match &handles.get(count) {
    //         None => break,
    //         Some(h) => ()
    //     } ;
    //         thread::sleep(time::Duration::from_secs(1));
    // }
    // for h in handles {
    //     let _ = h.join() ;
    // }
    // m.clear().unwrap();

    // let x = handles.into_iter().collect::<Vec<_>>() ;
    let (tx, _rx) = mpsc::channel(1000);

    {
        let mut set = JoinSet::new();
        for song in &songs {
            set.spawn(wrapped_build_pdf_song(
                tx.clone(),
                world.clone(),
                song.clone(),
                false,
            ));
            // break ;
        }
        log::info!("await");
        let _result = set.join_all().await;
        log::info!("done");
    }
    {
        let mut set = JoinSet::new();
        for book in &books {
            set.spawn(wrapped_build_pdf_book(
                tx.clone(),
                world.clone(),
                book.clone(),
            ));
            // break ;
        }
        log::info!("await");
        let _result = set.join_all().await;
        log::info!("done");
    }
    // let mut result = result
    //     .iter_mut()
    //     .filter_map(|r| match r {
    //         Ok(r) => Some(r),
    //         Err(e) => {
    //             log::error!("{:?}", e);
    //             None
    //         }
    //     })
    //     .collect::<Vec<_>>();
    //
    // loop {
    //     let result2 = result
    //         .iter_mut()
    //         .filter_map(|child| match child.try_wait() {
    //             Ok(status) => Some(status),
    //             Err(e) => {
    //                 log::error!("{:?}", e);
    //                 None
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //
    //     for r in &result2 {
    //         log::info!("{}:{} status : {:?}", file!(), line!(), r)
    //     }
    //
    //     let running = result2.iter().filter(|s| s.is_none()).collect::<Vec<_>>();
    //     log::info!("{} jobs running", &running.len());
    //     if running.len() == 0 {
    //         break;
    //     };
    //
    //     thread::sleep(Duration::from_millis(10_000));
    // }

    log::info!("build pdf songs done");
    // pb.finish_and_clear();

    // println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}
