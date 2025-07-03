use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::task::{ContextBuilder, LocalWaker, Poll, Waker};
use std::{
    io,
    time::{Duration, Instant},
};

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

use crate::ui::draw;

use crate::generate;
use crate::model::model::{LogItem, World};
use crate::ui::model::UiModel;

async fn reset<'a>(world: &World, set: &mut JoinSet<()>, uidata: &mut UiModel<'a>) {
    log::debug!("{}:{} reset", file!(), line!());
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

async fn rebuild<'a>(
    world: &World,
    nb_workers: u32,
    set: &mut JoinSet<()>,
    tx: Sender<LogItem>,
    uidata: &mut UiModel<'a>,
    force_rebuild: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // log::debug!("{}:{} rebuild; force={}", file!(), line!(), force_rebuild);
    std::thread::sleep(Duration::from_secs(1));

    let sb_to_start = uidata.run_n_songs_or_books(nb_workers);
    // log::debug!("{}:{} {:?}", file!(), line!(), &sb_to_start);

    for id in sb_to_start {
        let sb = uidata.get(id)?;
        sb.build_pdf(set, tx.clone(), world, force_rebuild);
    }

    Ok(())
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
) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_tick = Instant::now();
    // let mut set = JoinSet::new();
    // let mut count = 0;
    // let mut logsrunning: Vec<ListItem<'_>> = vec![];
    // let logsdone: Vec<ListItem<'_>> = vec![];

    let local_waker = LocalWaker::noop();
    let waker = Waker::noop();

    let mut cx = ContextBuilder::from_waker(waker)
        .local_waker(local_waker)
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
                }
                Poll::Ready(received) => match received {
                    None => {
                        log::error!("ready none");
                        break 'outer;
                    }
                    Some(s) => {
                        log::info!("{}:{} received something {:?}", file!(), line!(), &s);
                        uidata.handle_item(s, &mut f);
                        // break 'outer;
                    }
                },
            }
        }
        // uidata.make_logs();
        terminal.draw(|f| draw::drawx(f, &mut uidata))?;

        f.flush().unwrap();

        if needs_refresh {
            needs_refresh = false;
            log::debug!("refresh");
        }

        if needs_reset {
            reset(&world, &mut set, &mut uidata).await;
            needs_reset = false
        }

        // log::debug!("{}:{} {}",file!(),line!(),force_rebuild) ;
        if needs_rebuild || force_rebuild {
            // log::debug!("{}:{} {}", file!(), line!(), force_rebuild);
            needs_rebuild = false;
            rebuild(
                &world,
                nb_workers,
                &mut set,
                tx.clone(),
                &mut uidata,
                force_rebuild,
            )
            .await?;
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
                        writeln!(&mut f, " should quit ").unwrap();
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
