use console::Emoji;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::task::{ContextBuilder, LocalWaker, Poll, Waker};
use std::{
    io,
    time::{Duration, Instant},
};

// static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
// static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
// static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static ZZZ: Emoji<'_, '_> = Emoji("💤", "zzz");
static BEAR: Emoji<'_, '_> = Emoji("🐻", "bear");
// static LILY: Emoji<'_, '_> = Emoji("🪷", "lily");

use crate::actions::build_pdf::{wrapped_build_pdf_book, wrapped_build_pdf_song};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use human_sort::compare;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinSet;
use tokio::time::sleep;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::demo::ui;

use crate::generate;
use crate::model::model::{Book, ELogType, LogItem, Song, World};

async fn reset(
    world: &World,
    set: &mut JoinSet<()>,
    logsdone: &mut Vec<ListItem<'_>>,
    logsrunning: &mut Vec<ListItem<'_>>,
    pending_keys: &mut HashSet<(String, String)>,
    started_keys: &mut HashSet<(String, String)>,
    success_keys: &mut HashSet<(String, String)>,
    failure_keys: &mut HashSet<(String, String)>,
) {
    log::info!("{}:{} reset", file!(), line!());
    set.abort_all();
    loop {
        // log::info!("{}:{} {} loop", file!(), line!(), set.len());
        match set.join_next().await {
            None => break,
            Some(_) => (),
        };
    }

    match generate::all::generate_all(
        PathBuf::from(&world.songdir),
        PathBuf::from(&world.bookdir),
        PathBuf::from(&world.builddir),
    ) {
        Ok(()) => (),
        Err(e) => {
            log::error!("{}:{} {}", file!(), line!(), e);
            // println!("Custom backtrace: {}", Backtrace::force_capture());
            std::process::exit(1)
        }
    };

    pending_keys.clear();
    for song in world.songs.iter() {
        pending_keys.insert((song.author.clone(), song.title.clone()));
    }
    for book in world.books.iter() {
        pending_keys.insert(("BOOK".to_string(), book.title.clone()));
    }
    log::info!("{}:{} pending : {:?}", file!(), line!(), pending_keys);
    started_keys.clear();
    success_keys.clear();
    failure_keys.clear();
    logsdone.clear();
    logsrunning.clear();
    started_keys.clear();
    success_keys.clear();
    failure_keys.clear();
}

async fn rebuild(
    world: &World,
    nb_workers: u32,
    set: &mut JoinSet<()>,
    tx: Sender<LogItem>,
    pending_keys: &mut HashSet<(String, String)>,
    started_keys: &mut HashSet<(String, String)>,
    force_rebuild: bool,
) -> () {
    log::info!("{}:{} rebuild; force={}", file!(), line!(), force_rebuild);

    let n = nb_workers as i32 - started_keys.len() as i32;
    if n <= 0 {
        return ();
    }

    let songs_to_start = world
        .songs
        .iter()
        .filter(|&song| pending_keys.contains(&(song.author.clone(), song.title.clone())))
        .take(n as usize)
        .collect::<Vec<_>>();

    let books_to_start = if songs_to_start.len() == 0 && started_keys.len() == 0 {
        world
            .books
            .iter()
            .filter(|&book| pending_keys.contains(&("BOOK".to_string(), book.title.clone())))
            .take(n as usize)
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    log::info!(
        "{}:{} {} pending keys, {} running keys",
        file!(),
        line!(),
        pending_keys.len(),
        started_keys.len()
    );

    for song in songs_to_start {
        pending_keys.remove(&(song.author.clone(), song.title.clone()));
        started_keys.insert((song.author.clone(), song.title.clone()));
        let _ = set.spawn(wrapped_build_pdf_song(
            tx.clone(),
            world.clone(),
            song.clone(),
            force_rebuild,
        ));
    }

    for book in books_to_start {
        pending_keys.remove(&("BOOK".to_string(), book.title.clone()));
        started_keys.insert(("BOOK".to_string(), book.title.clone()));
        let _ = set.spawn(wrapped_build_pdf_book(
            tx.clone(),
            world.clone(),
            book.clone(),
        ));
    }

    // for book in &world.books {
    //     let _ = set.spawn(wrapped_build_pdf_book(
    //         tx.clone(),
    //         world.clone(),
    //         book.clone(),
    //     ));
    // }
}

//pub async fn run(world:World,tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
pub async fn run(
    world: World,
    nb_workers: u32,
    set: JoinSet<()>,
    tx: Sender<LogItem>,
    rx: &mut Receiver<LogItem>,
    // set: JoinSet<Result<(), Box<dyn std::error::Error>> >,
    tick_rate: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let _ = terminal.clear()?;

    let _res = run_appx(world, nb_workers, &mut terminal, set, tx, rx, tick_rate).await?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        &mut terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    let _ = &mut terminal.show_cursor()?;
    Ok(())
}

fn make_logs_done(
    sorted_songs: &Vec<Song>,
    sorted_books: &Vec<Book>,
    logsdone: &mut Vec<ListItem<'_>>,
    pending_keys: &HashSet<(String, String)>,
    started_keys: &HashSet<(String, String)>,
    failure_keys: &HashSet<(String, String)>,
    success_keys: &HashSet<(String, String)>,
) -> () {
    // log::info!("{}:{} make logs done",file!(),line!()) ;
    // let success_style = Style::default().bg(Color::Green);
    let failure_style = Style::default().bg(Color::Red);
    let author_style = Style::default().fg(Color::Blue);
    let book_style = Style::default().fg(Color::Yellow).bg(Color::White);
    let title_style = Style::default().fg(Color::Green);
    let running_style = Style::default().fg(Color::White);

    logsdone.clear();
    let mut ok_count = 0;
    let mut error_count = 0;
    let mut pending_count = 0;
    let mut started_count = 0;

    for song in sorted_songs {
        let content = if success_keys.contains(&(song.author.clone(), song.title.clone())) {
            // let content = vec![Spans::from(vec![
            //     Span::styled(format!("{}", SPARKLE), success_style),
            //     Span::raw(" "), // Span::styled(title, title_style),
            //     Span::styled(song.author.clone(), author_style),
            //     Span::raw(" @ "), // Span::styled(title, title_style),
            //     Span::styled(song.title.clone(), title_style),
            // ])];
            ok_count += 1;
            // Some(content)
            None
        } else if failure_keys.contains(&(song.author.clone(), song.title.clone())) {
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}", SPARKLE), failure_style),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled(song.author.clone(), author_style),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(song.title.clone(), title_style),
            ])];
            error_count += 1;
            Some(content)
        } else if pending_keys.contains(&(song.author.clone(), song.title.clone())) {
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}", BEAR), running_style),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled(song.author.clone(), author_style),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(song.title.clone(), title_style),
            ])];
            pending_count += 1;
            Some(content)
        } else if started_keys.contains(&(song.author.clone(), song.title.clone())) {
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}", TRUCK), running_style),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled(song.author.clone(), author_style),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(song.title.clone(), title_style),
            ])];
            started_count += 1;
            Some(content)
        } else {
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}", BEAR), running_style),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled(song.author.clone(), author_style),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(song.title.clone(), title_style),
            ])];
            Some(content)
        };

        match content {
            None => (),
            Some(content) => {
                let li = ListItem::new(content);
                logsdone.push(li)
            }
        };
    }
    for book in sorted_books {
        let content = if success_keys.contains(&("BOOK".to_string(), book.title.clone())) {
            // let content = vec![Spans::from(vec![
            //     Span::styled(format!("{}", SPARKLE), success_style),
            //     Span::raw(" "), // Span::styled(title, title_style),
            //     Span::styled(song.author.clone(), author_style),
            //     Span::raw(" @ "), // Span::styled(title, title_style),
            //     Span::styled(song.title.clone(), title_style),
            // ])];
            ok_count += 1;
            // Some(content)
            None
        } else if failure_keys.contains(&("BOOK".to_string(), book.title.clone())) {
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}", SPARKLE), failure_style),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled("BOOK".to_string(), book_style),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(book.title.clone(), title_style),
            ])];
            error_count += 1;
            Some(content)
        } else {
            let content = vec![Spans::from(vec![
                Span::styled(format!("{}", BEAR), running_style),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled("BOOK".to_string(), book_style),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(book.title.clone(), title_style),
            ])];
            Some(content)
        };

        match content {
            None => (),
            Some(content) => {
                let li = ListItem::new(content);
                logsdone.push(li)
            }
        };
    }
    {
        let content = vec![Spans::from(vec![
            // Span::styled(format!("{}", BEAR), running_style),
            Span::raw(format!(
                "{} Ok, {} Failures, {} Started, {} Pending ",
                ok_count, error_count, started_count, pending_count
            )), // Span::styled(title, title_style),
                // Span::styled(song.author.clone(), author_style),
                // Span::raw(" @ "), // Span::styled(title, title_style),
                // Span::styled(song.title.clone(), title_style),
        ])];
        let li = ListItem::new(content);
        logsdone.insert(0, li)
    }
}

fn handle_item(
    s: LogItem,
    f: &mut File,
    logsrunning: &mut Vec<ListItem<'_>>,
    started_keys: &mut HashSet<(String, String)>,
    failure_keys: &mut HashSet<(String, String)>,
    success_keys: &mut HashSet<(String, String)>,
) -> () {
    let success_style = Style::default().bg(Color::Green);
    let failure_style = Style::default().bg(Color::Red);
    let author_style = Style::default().fg(Color::Blue);
    let book_style = Style::default().fg(Color::Yellow).bg(Color::White);
    let title_style = Style::default().fg(Color::Green);
    let lily_style = Style::default().fg(Color::Cyan);
    let no_need_success_style = Style::default().fg(Color::LightGreen);
    let no_need_failed_style = Style::default().fg(Color::LightRed);
    // let thread_style = Style::default().fg(Color::Cyan).bg(Color::White);
    match &s {
        LogItem::Song(s) => {
            writeln!(f, "{:?}", &s.clone()).unwrap();
            match &s.status {
                ELogType::Started => {
                    let content = vec![Spans::from(vec![
                        Span::raw(format!("[  STARTED  ] ")), // Span::styled(title, title_style),
                        Span::styled(s.author.clone(), author_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Lualatex(count) => {
                    let ss = format!("[lualatex #{count}] ").clone();
                    let content = vec![Spans::from(vec![
                        Span::raw(ss), // Span::styled(title, title_style),
                        Span::styled(s.author.clone(), author_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Ps2pdf => {
                    let ss = format!("[   ps2pdf  ] ").clone();
                    let content = vec![Spans::from(vec![
                        Span::raw(ss), // Span::styled(title, title_style),
                        Span::styled(s.author.clone(), author_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Lilypond(ref lyfile) => {
                    let content = vec![Spans::from(vec![
                        Span::raw(format!("[  lilypond ] ")), // Span::styled(title, title_style),
                        Span::styled(s.author.clone(), author_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                        Span::raw(" ; "), // Span::styled(title, title_style),
                        // Span::raw(format!(" / {lyfile.clone()}").as_str()), // Span::styled(title, title_style),
                        Span::styled(lyfile.clone(), lily_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Success => {
                    success_keys.insert((s.author.clone(), s.title.clone()));
                    started_keys.remove(&(s.author.clone(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("   {}   ", SPARKLE), success_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled(s.author.clone(), author_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
                ELogType::Failed => {
                    failure_keys.insert((s.author.clone(), s.title.clone()));
                    started_keys.remove(&(s.author.clone(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("{}", SPARKLE), failure_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled(s.author.clone(), author_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
                ELogType::NoNeedSuccess => {
                    success_keys.insert((s.author.clone(), s.title.clone()));
                    started_keys.remove(&(s.author.clone(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("{}", ZZZ), no_need_success_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled(s.author.clone(), author_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
                ELogType::NoNeedFailed => {
                    success_keys.insert((s.author.clone(), s.title.clone()));
                    started_keys.remove(&(s.author.clone(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("{}", ZZZ), no_need_failed_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled(s.author.clone(), author_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
            }
        }
        LogItem::Book(s) => {
            writeln!(f, "{:?}", &s.clone()).unwrap();
            match &s.status {
                ELogType::Started => {
                    let content = vec![Spans::from(vec![
                        Span::raw(format!("[  STARTED  ] ")), // Span::styled(title, title_style),
                        Span::styled("BOOK", book_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Lualatex(count) => {
                    let ss = format!("[lualatex #{count}] ").clone();
                    let content = vec![Spans::from(vec![
                        Span::raw(ss), // Span::styled(title, title_style),
                        Span::styled("BOOK", book_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Ps2pdf => {
                    let ss = format!("[   ps2pdf  ] ").clone();
                    let content = vec![Spans::from(vec![
                        Span::raw(ss), // Span::styled(title, title_style),
                        Span::styled("BOOK", book_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Lilypond(ref lyfile) => {
                    let content = vec![Spans::from(vec![
                        Span::raw(format!("[  lilypond ] ")), // Span::styled(title, title_style),
                        Span::styled("BOOK", book_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(s.title.clone(), title_style),
                        Span::raw(" ; "), // Span::styled(title, title_style),
                        // Span::raw(format!(" / {lyfile.clone()}").as_str()), // Span::styled(title, title_style),
                        Span::styled(lyfile.clone(), lily_style),
                    ])];
                    let li = ListItem::new(content);
                    logsrunning.insert(0, li);
                }
                ELogType::Success => {
                    success_keys.insert(("BOOK".to_string(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("   {}   ", SPARKLE), success_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled("BOOK", book_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
                ELogType::Failed => {
                    failure_keys.insert(("BOOK".to_string(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("{}", SPARKLE), failure_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled("BOOK", book_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
                ELogType::NoNeedSuccess => {
                    success_keys.insert(("BOOK".to_string(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("{}", ZZZ), no_need_success_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled("BOOK", book_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
                ELogType::NoNeedFailed => {
                    success_keys.insert(("BOOK".to_string(), s.title.clone()));
                    {
                        let content = vec![Spans::from(vec![
                            Span::styled(format!("{}", ZZZ), no_need_failed_style),
                            Span::raw(" "), // Span::styled(title, title_style),
                            Span::styled("BOOK", book_style),
                            Span::raw(" @ "), // Span::styled(title, title_style),
                            Span::styled(s.title.clone(), title_style),
                        ])];
                        let li = ListItem::new(content);
                        logsrunning.insert(0, li);
                    }
                }
            }
        }
    };

    // log::info!("{}:{} {} logsrunning", file!(), line!(), logsrunning.len());

    // while logsrunning.len() > 20 {
    //     logsrunning.pop();
    // }
}

async fn run_appx<B: Backend>(
    world: World,
    nb_workers: u32,
    terminal: &mut Terminal<B>,
    mut set: JoinSet<()>,
    tx: Sender<LogItem>,
    rx: &mut Receiver<LogItem>,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    // let mut set = JoinSet::new();
    // let mut count = 0;
    let mut logsrunning: Vec<ListItem<'_>> = vec![];
    let mut logsdone: Vec<ListItem<'_>> = vec![];

    let local_waker = LocalWaker::noop();
    let waker = Waker::noop();

    let mut cx = ContextBuilder::from_waker(&waker)
        .local_waker(&local_waker)
        .build();

    let mut sorted_songs = world.songs.clone();
    sorted_songs.sort_by(|a, b| match compare(a.author.as_str(), b.author.as_str()) {
        Ordering::Equal => compare(a.title.as_str(), b.title.as_str()),
        x => x,
    });

    let mut sorted_books = world.books.clone();
    sorted_books.sort_by(|a, b| compare(a.title.as_str(), b.title.as_str()));

    let mut needs_rebuild = true;
    let mut force_rebuild = false;
    let mut needs_reset = false;
    let mut should_quit = false;
    let mut needs_refresh = false;
    let mut success_keys: HashSet<(String, String)> = HashSet::new();
    let mut failure_keys: HashSet<(String, String)> = HashSet::new();
    let mut all_keys: HashSet<(String, String)> = HashSet::new();
    let mut pending_keys: HashSet<(String, String)> = HashSet::new();
    let mut started_keys: HashSet<(String, String)> = HashSet::new();
    for song in &world.songs {
        all_keys.insert((song.author.clone(), song.title.clone()));
    }
    for book in &world.books {
        all_keys.insert(("BOOK".to_string(), book.title.clone()));
    }

    let mut f = File::options()
        .append(true)
        .create(true)
        .open("date.log")
        .unwrap();

    loop {
        // let x = rx.poll_recv(cx) ;
        // let received = rx.recv().await;

        // while !rx.is_empty() {
        //     log::info!("received something");
        //     match rx.blocking_recv() {
        //         None => {
        //             log::error!("rx not empty but recv is None")
        //         }
        //         Some(s) => handle_item(
        //             s,
        //             &mut f,
        //             &mut logsrunning,
        //             &mut failure_keys,
        //             &mut success_keys,
        //         ),
        //     }
        // }

        'outer: loop {
            let preceived = rx.poll_recv(&mut cx);
            match preceived {
                Poll::Pending => {
                    if !rx.is_empty() {
                        log::error!("pending but not empty : {}", rx.len());
                        let _x = sleep(Duration::from_millis(10)).await;
                    } else {
                        break 'outer;
                    }
                    // log::info!("message pending");
                    ()
                }
                Poll::Ready(received) => match received {
                    None => {
                        log::error!("ready none");
                        break 'outer;
                    }
                    Some(s) => {
                        log::info!("{}:{} received something {:?}", file!(), line!(), &s);
                        handle_item(
                            s,
                            &mut f,
                            &mut logsrunning,
                            &mut started_keys,
                            &mut failure_keys,
                            &mut success_keys,
                        );
                        break 'outer;
                    }
                },
            }
        }
        make_logs_done(
            &sorted_songs,
            &sorted_books,
            &mut logsdone,
            &pending_keys,
            &started_keys,
            &failure_keys,
            &success_keys,
        );
        terminal.draw(|f| ui::drawx(f, logsrunning.clone(), logsdone.clone()))?;

        f.flush().unwrap();

        if needs_refresh {
            needs_refresh = false;
            log::info!("refresh");
        }

        if needs_reset {
            reset(
                &world,
                &mut set,
                &mut logsdone,
                &mut logsrunning,
                &mut pending_keys,
                &mut started_keys,
                &mut success_keys,
                &mut failure_keys,
            )
            .await;
            needs_reset = false
        }

        // log::info!("{}:{} {}",file!(),line!(),force_rebuild) ;
        if needs_rebuild || force_rebuild {
            log::info!("{}:{} {}", file!(), line!(), force_rebuild);
            needs_rebuild = false;
            rebuild(
                &world,
                nb_workers,
                &mut set,
                tx.clone(),
                &mut pending_keys,
                &mut started_keys,
                force_rebuild,
            )
            .await;
            force_rebuild = false
        }

        // terminal.draw(|f| ui::drawx(f, logs.clone()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(1));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // KeyCode::Char(c) => app.on_key(c),
                    // KeyCode::Left => app.on_left(),
                    // KeyCode::Up => app.on_up(),
                    // KeyCode::Right => app.on_right(),
                    // KeyCode::Down => app.on_down(),
                    KeyCode::Char('q') => {
                        let _ = writeln!(&mut f, "{}", " should quit ").unwrap();
                        should_quit = true
                    }
                    // KeyCode::Char('c') => check_now = true,
                    KeyCode::Char('b') => needs_rebuild = true,
                    KeyCode::Char('s') => needs_reset = true,
                    KeyCode::Char('f') => {
                        force_rebuild = true;
                        needs_rebuild = true
                    }
                    KeyCode::Char('r') => needs_refresh = true,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            if (started_keys.len() as u32) < nb_workers && pending_keys.len() > 0 {
                needs_rebuild = true;
            }

            last_tick = Instant::now();
        }
        if should_quit {
            return Ok(());
        }
    }
}

// async fn run_app<B: Backend>(
//     terminal: &mut Terminal<B>,
//     mut app: App<'_>,
//     tick_rate: Duration,
// ) -> io::Result<()> {
//     let mut last_tick = Instant::now();
//     // let mut set = JoinSet::new();
//
//     loop {
//         terminal.draw(|f| ui::draw(f, &mut app))?;
//
//         let timeout = tick_rate
//             .checked_sub(last_tick.elapsed())
//             .unwrap_or_else(|| Duration::from_secs(0));
//         if crossterm::event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 match key.code {
//                     KeyCode::Char(c) => app.on_key(c),
//                     KeyCode::Left => app.on_left(),
//                     KeyCode::Up => app.on_up(),
//                     KeyCode::Right => app.on_right(),
//                     KeyCode::Down => app.on_down(),
//                     _ => {}
//                 }
//             }
//         }
//
//         // for song in &world.songs {
//         //     set.spawn(build_pdf(
//         //         song.clone(),
//         //     ));
//         //     // break ;
//         // }
//
//         if last_tick.elapsed() >= tick_rate {
//             app.on_tick();
//             last_tick = Instant::now();
//         }
//         if app.should_quit {
//             return Ok(());
//         }
//     }
// }
