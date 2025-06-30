use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::task::{ContextBuilder, LocalWaker, Poll, Waker};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::actions::build_pdf::{wrapped_build_pdf_book, wrapped_build_pdf_song};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinSet;
use tokio::time::sleep;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::ui::ui;

use crate::generate;
use crate::model::model::{LogItem, World};
use crate::ui::model::UiModel;

async fn reset(world: &World, set: &mut JoinSet<()>, uidata: &mut UiModel<'_>) {
    log::info!("{}:{} reset", file!(), line!());
    set.abort_all();
    loop {
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

    uidata.init();
}

async fn rebuild(
    world: &World,
    nb_workers: u32,
    set: &mut JoinSet<()>,
    tx: Sender<LogItem>,
    uidata: &mut UiModel<'_>,
    force_rebuild: bool,
) -> () {
    // log::info!("{}:{} rebuild; force={}", file!(), line!(), force_rebuild);

    let (songs_to_start, books_to_start) = uidata.run_n_songs_or_books(nb_workers);

    for song in songs_to_start {
        let _ = set.spawn(wrapped_build_pdf_song(
            tx.clone(),
            world.clone(),
            song.clone(),
            force_rebuild,
        ));
    }

    for book in books_to_start {
        let _ = set.spawn(wrapped_build_pdf_book(
            tx.clone(),
            world.clone(),
            book.clone(),
        ));
    }
}

//pub async fn run(world:World,tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
pub async fn run(
    world: World,
    nb_workers: u32,
    set: JoinSet<()>,
    tx: Sender<LogItem>,
    rx: &mut Receiver<LogItem>,
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
    // let mut logsrunning: Vec<ListItem<'_>> = vec![];
    // let logsdone: Vec<ListItem<'_>> = vec![];

    let local_waker = LocalWaker::noop();
    let waker = Waker::noop();

    let mut cx = ContextBuilder::from_waker(&waker)
        .local_waker(&local_waker)
        .build();

    let mut needs_rebuild = true;
    let mut force_rebuild = false;
    let mut needs_reset = false;
    let mut should_quit = false;
    let mut needs_refresh = false;

    let mut uidata = UiModel::new(&world);

    let mut f = File::options()
        .append(true)
        .create(true)
        .open("date.log")
        .unwrap();

    loop {
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
                    ()
                }
                Poll::Ready(received) => match received {
                    None => {
                        log::error!("ready none");
                        break 'outer;
                    }
                    Some(s) => {
                        log::info!("{}:{} received something {:?}", file!(), line!(), &s);
                        uidata.handle_item(s, &mut f);
                        break 'outer;
                    }
                },
            }
        }
        uidata.make_logs();
        terminal.draw(|f| ui::drawx(f, &mut uidata))?;

        f.flush().unwrap();

        if needs_refresh {
            needs_refresh = false;
            log::info!("refresh");
        }

        if needs_reset {
            reset(&world, &mut set, &mut uidata).await;
            needs_reset = false
        }

        // log::info!("{}:{} {}",file!(),line!(),force_rebuild) ;
        if needs_rebuild || force_rebuild {
            // log::info!("{}:{} {}", file!(), line!(), force_rebuild);
            needs_rebuild = false;
            rebuild(
                &world,
                nb_workers,
                &mut set,
                tx.clone(),
                &mut uidata,
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
            // if (started_keys.len() as u32) < nb_workers && pending_keys.len() > 0 {
            //     needs_rebuild = true;
            // }
            needs_rebuild = true;

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
