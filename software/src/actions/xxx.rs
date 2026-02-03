use colored_text::Colorize;
use log;
// use petgraph::data::Build;

use std::path::PathBuf;
use tokio::sync::mpsc;

use indicatif::{ProgressBar, ProgressStyle};

use crate::actions::graph::build_graph;
use crate::actions::{
    run_bootstrap_book, run_bootstrap_song, run_deliver_book, run_deliver_song, run_fluidsynth,
    run_lilypond_midi, run_lilypond_snippet, run_lualatex_book, run_lualatex_song, run_mountfile,
};
use crate::model::use_model as M;
use crate::model::use_model::LogItem;

use crate::actions::graph as G;
use crate::model::world::make;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use std::result::Result;
// use std::time::Duration;

use tokio::sync::mpsc::Receiver;
use tokio::task::JoinSet;

pub async fn build_node(
    tx: mpsc::Sender<(NodeIndex, M::BuildType)>,
    world: M::World,
    node: G::Artefact,
    ni: NodeIndex,
    deps: Vec<PathBuf>,
) -> () {
    log::info!("working on {:?} {:?}", node, deps);

    let bt: M::BuildType = match node.kind {
        G::EArtefact::SongPdf(song) => {
            match run_lualatex_song::run(world.clone(), song.clone(), deps).await {
                Ok(b) => b,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::DeliverySongPdf(song) => {
            match run_deliver_song::run(world.clone(), song.clone(), deps).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::MountedFile(song, tfex) => {
            match run_mountfile::run(world.clone(), song.clone(), tfex.clone()).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::LySnippet(song, ly) => {
            match run_lilypond_snippet::run(world.clone(), song.clone(), ly.clone(), deps).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::Midi(song, ly) => {
            match run_lilypond_midi::run(world.clone(), song.clone(), ly.clone(), deps).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::Wav(song, ly) => {
            match run_fluidsynth::run(world.clone(), song.clone(), ly.clone(), deps).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }

        G::EArtefact::BookPdf(book) => {
            match run_lualatex_book::run(world.clone(), book.clone(), deps).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("in run_lualatex_book {:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::DeliveryBookPdf(book) => {
            match run_deliver_book::run(world.clone(), book.clone(), deps).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("{:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::All => {
            let mut target = world.builddir.clone();
            target.push("all");
            M::BuildType::Rebuilt(target)
        }
        G::EArtefact::BootstrapSong(song) => {
            match run_bootstrap_song::run(world.clone(), song.clone()).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("bootstrap : {:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::BootstrapBook(book) => {
            match run_bootstrap_book::run(world.clone(), book.clone()).await {
                Ok(bt) => bt,
                Err(_e) => {
                    log::error!("bootstrap : {:?}", _e);
                    M::BuildType::Failed
                }
            }
        }
        G::EArtefact::Broken(_path) => M::BuildType::Failed,
    };
    match tx.send((ni, bt)).await {
        Ok(()) => {
            // log::info!("ok, sent");
            ()
        }
        Err(e) => log::error!("failed to send node index: {:?} {}", ni, e),
    };
}

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
    _force_rebuild: bool,
    nb_workers: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // match generate_all(songdir.clone(), bookdir.clone(), builddir.clone()) {
    //     Ok(()) => (),
    //     Err(e) => return Result::Err(e),
    // }
    let world = make(&songdir, &bookdir, &builddir)?;

    // let world: M::World = {
    //     let mut path = PathBuf::from(&builddir);
    //     path.push("world-internal.json");
    //     let data = std::fs::read_to_string(path.to_str().unwrap()).unwrap();
    //     serde_json::from_str(data.as_str()).unwrap()
    // };

    let (tx, mut rx) = mpsc::channel::<(NodeIndex, M::BuildType)>(1000);

    let mut set: JoinSet<()> = JoinSet::new();

    let g: petgraph::Graph<G::Artefact, crate::actions::graph::Route> = build_graph(&world)?;

    // let done_text = "DONE".hex("#8B008B").on_hex("#7FFF00").bold();
    let done_text = "DONE".hex("#7FFF00").bold();
    // let not_touched_text = "Skip".hex("#8B008B").on_hex("#7FFFFF").bold();
    let failed_text = "FAILED".hex("#FF1493").on_hex("#F0FFFF").bold();
    let ancestor_failed_text = "Ancestor Failed".hex("#FF8C00").on_hex("#000000").bold();

    let pb = ProgressBar::new(g.node_indices().count().try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template(
            "PDF  [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    let mut pending: HashSet<NodeIndex> = HashSet::new();
    let mut running: HashSet<NodeIndex> = HashSet::new();
    let mut rebuilt: HashSet<NodeIndex> = HashSet::new();
    let mut built_targets: HashMap<NodeIndex, PathBuf> = HashMap::new();
    let mut skipped: HashSet<NodeIndex> = HashSet::new();
    let mut failed: HashSet<NodeIndex> = HashSet::new();
    let mut ancestor_failed: HashSet<NodeIndex> = HashSet::new();

    for n in g.node_indices() {
        log::info!("{:?} : {:?}", n, g.node_weight(n));
        let nk = g.node_weight(n).ok_or("huh, no node?")?;
        match nk.kind {
            G::EArtefact::All
            | G::EArtefact::BootstrapSong(_)
            | G::EArtefact::BootstrapBook(_)
            | G::EArtefact::SongPdf(_)
            | G::EArtefact::DeliverySongPdf(_)
            | G::EArtefact::MountedFile(_, _)
            | G::EArtefact::LySnippet(_, _)
            | G::EArtefact::Midi(_, _)
            | G::EArtefact::Wav(_, _)
            | G::EArtefact::BookPdf(_)
            | G::EArtefact::DeliveryBookPdf(_) => pending.insert(n),
            G::EArtefact::Broken(_) => failed.insert(n),
        };
    }

    let total_nodes = g.node_indices().count();
    'outermost: loop {
        log::info!(
            "{} pending ; {} running ; {} rebuilt ; {} failed, {} ancestor_failed, {} skipped, nb_workers={} ; total_nodes={}",
            pending.len(),
            running.len(),
            rebuilt.len(),
            failed.len(),
                ancestor_failed.len(),
            skipped.len(),
            nb_workers,
            total_nodes
        );
        if pending.len()
            + running.len()
            + rebuilt.len()
            + failed.len()
            + ancestor_failed.len()
            + skipped.len()
            != total_nodes
        {
            log::error!("logic error here");
            log::error!("pending: {:?}", pending);
            log::error!("running: {:?}", running);
            log::error!("rebuilt: {:?}", rebuilt);
            log::error!("failed: {:?}", failed);
            log::error!("ancestor_failed: {:?}", ancestor_failed);
            log::error!("skipped: {:?}", skipped);
            assert!(false);
        }

        if total_nodes == rebuilt.len() + failed.len() + ancestor_failed.len() + skipped.len() {
            log::info!("breaking outermost because condition reached");
            break 'outermost;
        }

        'outer: loop {
            // log::info!("running: {:?}", running.len());
            // if running.len() == nb_workers as usize {
            //     log::info!("break 'outer");
            //     break 'outer;
            // }
            if pending.is_empty() {
                break 'outer;
            }

            loop {
                for n in g.node_indices() {
                    if running.len() == nb_workers as usize {
                        log::info!("break 'outer");
                        break 'outer;
                    }
                    if !pending.contains(&n) {
                        continue;
                    }
                    let node = g.node_weight(n).ok_or("huh, no node?")?;
                    let mut ok_to_start = true;
                    let mut an_ancestor_failed = false;
                    let mut an_ancestor_changed = false;

                    for p in g.neighbors_directed(n, petgraph::Direction::Incoming) {
                        if !rebuilt.contains(&p) && !skipped.contains(&p) {
                            ok_to_start = false;
                        }
                        if failed.contains(&p) || ancestor_failed.contains(&p) {
                            an_ancestor_failed = true;
                        }
                        if rebuilt.contains(&p) {
                            an_ancestor_changed = true;
                        }
                    }
                    if an_ancestor_failed {
                        log::info!("ANCESTOR FAILED === > {:?}", node);
                        pending.remove(&n);
                        // ancestor_failed.insert(n);
                        match tx.send((n, M::BuildType::AncestorFailed)).await {
                            Ok(()) => {
                                // log::info!("ok, sent");
                                ()
                            }
                            Err(e) => log::error!("failed to send node index: {:?} {}", n, e),
                        };
                    // } else if ok_to_start && !an_ancestor_changed {
                    //     log::info!("SKIP === > {:?}", node);
                    //     pending.remove(&n);
                    //     skipped.insert(n);
                    //     match tx
                    //         .send((n, M::BuildType::NotTouched(PathBuf::from(""))))
                    //         .await
                    //     {
                    //         Ok(()) => {
                    //             // log::info!("ok, sent");
                    //             ()
                    //         }
                    //         Err(e) => log::error!("failed to send node index: {:?} {}", n, e),
                    //     };
                    } else if ok_to_start {
                        log::info!("START === > node {:?} ", node);
                        pending.remove(&n);
                        running.insert(n);
                        let deps = g
                            .neighbors_directed(n, petgraph::Direction::Incoming)
                            .map(|ni| built_targets.get(&ni).ok_or("huh ?"))
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .map(|x| x.clone())
                            .collect::<Vec<_>>();

                        // tx.send(n).await.unwrap();
                        set.spawn(build_node(tx.clone(), world.clone(), node.clone(), n, deps));
                        // break 'outer;
                    } else {
                        log::info!("node not ready : {:?} ; ok_to_start:{}, an_ancestor_failed:{}, an_ancestor_changed:{}", node,
                    ok_to_start,an_ancestor_failed,an_ancestor_changed);
                    }
                }
                break 'outer;
            }
        }
        if let Some(li) = rx.recv().await {
            log::info!("{:?}", &li);
            running.remove(&li.0);
            let node = g.node_weight(li.0).ok_or("huh, no node?")?;
            match li.1 {
                M::BuildType::Rebuilt(target) => {
                    rebuilt.insert(li.0);
                    built_targets.insert(li.0, target);
                    pb.println(format!("{} node {:?} ", done_text, node));
                }
                M::BuildType::NotTouched(target) => {
                    skipped.insert(li.0);
                    built_targets.insert(li.0, target);
                    // pb.println(format!("{} node {:?} ", not_touched_text, node));
                }
                M::BuildType::AncestorFailed => {
                    ancestor_failed.insert(li.0);
                    pb.println(format!("{} node {:?} ", failed_text, node));
                }
                M::BuildType::Failed => {
                    failed.insert(li.0);
                    pb.println(format!("{} node {:?} ", failed_text, node));
                }
            };
            pb.inc(1);
        }
    }

    for ni in ancestor_failed {
        let node = g.node_weight(ni).ok_or("huh, no node?")?;
        pb.println(format!("{} node {:?} ", ancestor_failed_text, node));
    }

    for ni in failed {
        let node = g.node_weight(ni).ok_or("huh, no node?")?;
        pb.println(format!("{} node {:?} ", failed_text, node));
    }

    // std::thread::sleep(Duration::from_secs(10000));
    // unimplemented!("moving to graph");
    // // log::info!("calling generate_for_aws_lambda");
    // // generate::generate::generate_for_aws_lambda(&PathBuf::from(&world.builddir)).unwrap();

    // let (tx, mut rx) = mpsc::channel::<LogItem>(1000);

    // let mut songs_to_build = world.songs.clone();
    // let mut books_to_build = world.books.clone();

    // let count_songs_to_do = songs_to_build.len();
    // let count_books_to_do = books_to_build.len();

    // let mut count_songs_done = 0;
    // let mut running_songs: Vec<Song> = vec![];
    // let mut running_books: Vec<Book> = vec![];
    // let mut count_books_done = 0;

    // let pb = ProgressBar::new((count_songs_to_do + count_books_to_do) as u64);
    // pb.set_style(
    //     ProgressStyle::with_template(
    //         "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    //     )
    //     .unwrap(),
    // );
    // // let pb = ProgressBar::new_spinner();
    // // pb.enable_steady_tick(Duration::from_millis(120));
    // // pb.set_style(
    // //     ProgressStyle::with_template("{spinner:.blue} {msg}")
    // //         .unwrap()
    // //         // For more spinners check out the cli-spinners project:
    // //         // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
    // //         .tick_strings(&[
    // //             "▹▹▹▹▹",
    // //             "▸▹▹▹▹",
    // //             "▹▸▹▹▹",
    // //             "▹▹▸▹▹",
    // //             "▹▹▹▸▹",
    // //             "▹▹▹▹▸",
    // //             "▪▪▪▪▪",
    // //         ]),
    // // );

    // loop {
    //     if (running_songs.len() as u32) < nb_workers {
    //         if let Some(song) = songs_to_build.pop() {
    //             log::info!("RUN {} {}", song.author, song.title);
    //             running_songs.push(song.clone());
    //             pb.println(cformat!(
    //                 "<blue>START</blue> song <green>{}</green> @ <cyan>{}</cyan>",
    //                 song.author,
    //                 song.title
    //             ));
    //             set.spawn(wrapped_build_pdf_song(
    //                 tx.clone(),
    //                 world.clone(),
    //                 song.clone(),
    //                 force_rebuild,
    //             ));
    //         }
    //     }
    //     if count_songs_done == count_songs_to_do && (running_books.len() as u32) < nb_workers {
    //         if let Some(book) = books_to_build.pop() {
    //             log::info!("BOOK {}", book.title);
    //             running_books.push(book.clone());
    //             pb.println(cformat!(
    //                 "<blue>START</blue> book  <cyan>{}</cyan>",
    //                 book.title
    //             ));
    //             set.spawn(wrapped_build_pdf_book(
    //                 tx.clone(),
    //                 world.clone(),
    //                 book.clone(),
    //             ));
    //         }
    //     }

    //     if let Some(li) = rx.recv().await {
    //         log::info!("{}:{} {:?}", file!(), line!(), li);
    //         match li {
    //             LogItem::Book(b) => match b.status {
    //                 ELogType::Success
    //                 | ELogType::Failed
    //                 | ELogType::NoNeedFailed
    //                 | ELogType::NoNeedSuccess => {
    //                     count_books_done += 1;
    //                     running_books = running_books
    //                         .into_iter()
    //                         .filter(|book| b.title != book.title)
    //                         .collect::<Vec<_>>();
    //                     pb.println(cformat!(
    //                         "<green!>DONE</green!> book <cyan>{}</cyan>",
    //                         b.title
    //                     )); // pb.set_message(format!("... > book {} ", b.title));

    //                     pb.inc(1);
    //                 }
    //                 ELogType::Started
    //                 | ELogType::Ps2pdf
    //                 | ELogType::Lilypond(_)
    //                 | ELogType::Wav(_)
    //                 | ELogType::Lualatex(_) => {}
    //             },
    //             LogItem::Song(s) => {
    //                 match s.status {
    //                     ELogType::Success | ELogType::NoNeedSuccess => {
    //                         count_songs_done += 1;
    //                         let before = running_songs.len();
    //                         running_songs = running_songs
    //                             .into_iter()
    //                             .filter(|song| s.author != song.author || s.title != song.title)
    //                             .collect::<Vec<_>>();
    //                         let after = running_songs.len();
    //                         if after != before - 1 {
    //                             log::error!("logic error here {s:?}");
    //                         }
    //                         // assert!(count_done + running_songs.len() == count_to_do);
    //                         // pb.set_message(format!("... > song {} @ {}", s.author, s.title));
    //                         pb.println(cformat!(
    //                             "<green!>DONE</green!> song <green>{}</green> @ <cyan>{}</cyan>",
    //                             s.author,
    //                             s.title
    //                         ));

    //                         pb.inc(1);
    //                     }
    //                     ELogType::Failed | ELogType::NoNeedFailed => {
    //                         count_songs_done += 1;
    //                         let before = running_songs.len();
    //                         running_songs = running_songs
    //                             .into_iter()
    //                             .filter(|song| s.author != song.author || s.title != song.title)
    //                             .collect::<Vec<_>>();
    //                         let after = running_songs.len();
    //                         if after != before - 1 {
    //                             log::error!("logic error here {s:?}");
    //                         }
    //                         // assert!(count_done + running_songs.len() == count_to_do);
    //                         // pb.set_message(format!("... > song {} @ {}", s.author, s.title));
    //                         pb.println(cformat!(
    //                             "<red!>FAILED</red!> song <green>{}</green> @ <cyan>{}</cyan>",
    //                             s.author,
    //                             s.title
    //                         ));

    //                         pb.inc(1);
    //                     }
    //                     ELogType::Lilypond(ly) => {
    //                         pb.println(format!("lilypond {} @ {} : {}", s.author, s.title, ly));
    //                     }
    //                     ELogType::Wav(wav) => {
    //                         pb.println(format!("wav {} @ {} : {}", s.author, s.title, wav));
    //                     }
    //                     ELogType::Lualatex(i) => {
    //                         pb.println(format!("lualatex #{i}  {} @ {} ", s.author, s.title));
    //                     }
    //                     ELogType::Started => {
    //                         pb.println(format!("Started  {} @ {} ", s.author, s.title));
    //                     }
    //                     ELogType::Ps2pdf => {
    //                         pb.println(format!("Ps2pdf  {} @ {} ", s.author, s.title));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     if count_songs_done == count_songs_to_do && count_books_done == count_books_to_do {
    //         break;
    //     }
    // }
    pb.finish_with_message("done");

    Ok(())
}
