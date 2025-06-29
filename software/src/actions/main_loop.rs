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
use crate::model::use_model::World;
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

pub async fn main_loop(world: &World) {
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
        log::debug!("await");
        let _result = set.join_all().await;
        log::debug!("done");
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
        log::debug!("await");
        let _result = set.join_all().await;
        log::debug!("done");
    }

    log::debug!("build pdf songs done");
    // pb.finish_and_clear();

    // println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}
