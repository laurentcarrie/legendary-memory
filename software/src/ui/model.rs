use crate::actions::build_pdf::{wrapped_build_pdf_book, wrapped_build_pdf_song};
use console::Emoji;
use human_sort::compare;
use std::cmp::Ordering;
use std::fs::File;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;

static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
// static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
// static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
// static ZZZ: Emoji<'_, '_> = Emoji("💤", "zzz");
static BEAR: Emoji<'_, '_> = Emoji("🐻", "bear");

use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;

use std::cmp;

use crate::model::model::{Book, ELogType, LogItem, LogItemBook, LogItemSong, Song, World};

#[derive(PartialEq, Clone, Debug)]
pub enum Estate {
    Pending,
    Running,
    Success,
    Failure,
}

#[derive(PartialEq, Clone, Debug)]
pub struct UISong {
    song: Song,
    state: Estate,
}

impl UISong {
    pub fn new(song: Song) -> UISong {
        UISong {
            song,
            state: Estate::Pending,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct UIBook {
    book: Book,
    state: Estate,
}

impl UIBook {
    pub fn new(book: Book) -> UIBook {
        UIBook {
            book,
            state: Estate::Pending,
        }
    }
}

pub trait TEstate {
    fn id(&self) -> String;
    fn state(&self) -> Estate;
    fn build_level(&self) -> u32;
    fn change_state(&mut self, state: Estate);
    fn uicontent<'a>(&self) -> Option<Vec<Spans<'a>>>;
    fn build_pdf(
        &self,
        set: &mut JoinSet<()>,
        tx: Sender<LogItem>,
        world: &World,
        force_rebuild: bool,
    );
}

impl TEstate for UISong {
    fn id(&self) -> String {
        format!("{} @ {}", self.song.author.clone(), self.song.title.clone())
    }
    /// the current building state of a book or a song pdf
    fn state(&self) -> Estate {
        self.state.clone()
    }
    fn build_level(&self) -> u32 {
        0
    }
    fn change_state(&mut self, state: Estate) {
        self.state = state
    }
    /// how to display
    fn uicontent<'a>(&self) -> Option<Vec<Spans<'a>>> {
        let style = |emoji: Emoji<'_, '_>| {
            vec![Spans::from(vec![
                Span::styled(format!("{}", emoji), Style::default()),
                Span::raw(" "),
                Span::styled(self.song.author.clone(), Style::default().fg(Color::Blue)),
                Span::raw(" @ "),
                Span::styled(self.song.title.clone(), Style::default().fg(Color::Green)),
            ])]
        };
        match self.state {
            Estate::Pending => Some(style(BEAR)),
            Estate::Running => Some(style(TRUCK)),
            Estate::Success => None,
            Estate::Failure => Some(style(SPARKLE)),
        }
    }

    fn build_pdf(
        &self,
        set: &mut JoinSet<()>,
        tx: Sender<LogItem>,
        world: &World,
        force_rebuild: bool,
    ) {
        let _ = set.spawn(wrapped_build_pdf_song(
            tx.clone(),
            world.clone(),
            self.song.clone(),
            force_rebuild,
        ));
    }
}

impl TEstate for UIBook {
    fn id(&self) -> String {
        format!("BOOK @ {}", self.book.title.clone())
    }

    fn state(&self) -> Estate {
        self.state.clone()
    }
    fn build_level(&self) -> u32 {
        1
    }
    fn change_state(&mut self, state: Estate) {
        self.state = state
    }
    fn uicontent<'a>(&self) -> Option<Vec<Spans<'a>>> {
        let style = |emoji: Emoji<'_, '_>| {
            vec![Spans::from(vec![
                Span::styled(format!("{}", emoji), Style::default()),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled("BOOK", Style::default().fg(Color::Blue)),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(self.book.title.clone(), Style::default().fg(Color::Green)),
            ])]
        };

        match self.state {
            Estate::Success => None,
            Estate::Pending => Some(style(BEAR)),
            Estate::Running => Some(style(TRUCK)),
            Estate::Failure => Some(style(SPARKLE)),
        }
    }

    fn build_pdf(
        &self,
        set: &mut JoinSet<()>,
        tx: Sender<LogItem>,
        world: &World,
        _force_rebuild: bool,
    ) {
        let _ = set.spawn(wrapped_build_pdf_book(
            tx.clone(),
            world.clone(),
            self.book.clone(),
        ));
    }
}
// unimplemented!()

pub struct UiModel<'a> {
    //  set: JoinSet<()>,
    // songs: Vec<(Song, Estate)>,
    // books: Vec<(Book, Estate)>,
    // logsdone: Vec<ListItem<'a>>,
    logsrunning: Vec<ListItem<'a>>,
    songs_and_books: Vec<Box<dyn TEstate>>,
}

// impl<'a> UiModel<'a> {
//     pub fn new(world: &World) -> UiModel<'a> {
impl<'a> UiModel<'a> {
    pub fn new(world: &World) -> UiModel {
        let mut sorted_songs = world
            .songs
            .clone()
            .iter()
            .map(|song| UISong::new(song.clone()))
            .collect::<Vec<_>>();
        sorted_songs.sort_by(|a, b| {
            match compare(a.song.author.as_str(), b.song.author.as_str()) {
                Ordering::Equal => compare(a.song.title.as_str(), b.song.title.as_str()),
                x => x,
            }
        });

        let mut sorted_books = world
            .books
            .clone()
            .iter()
            .map(|book| UIBook::new(book.clone()))
            .collect::<Vec<_>>();
        sorted_books.sort_by(|a, b| compare(a.book.title.as_str(), b.book.title.as_str()));

        let mut sb: Vec<Box<dyn TEstate>> = vec![];
        for s in sorted_songs {
            sb.push(Box::new(s))
        }
        for b in sorted_books {
            sb.push(Box::new(b))
        }

        UiModel {
            // songs: sorted_songs.clone(),
            // books: sorted_books.clone(),
            // set: JoinSet::new(),
            songs_and_books: sb,
            logsrunning: vec![],
        }
    }

    pub fn init(&mut self) {
        for sb in &mut self.songs_and_books {
            sb.change_state(Estate::Pending);
        }
    }

    /// if number of running builds < nb_workers, take n from pending and return them.
    /// as long as all songs are not built, ignore the books
    pub fn run_n_songs_or_books(&mut self, nb_workers: u32) -> Vec<String> {
        let nb_running = self
            .songs_and_books
            .iter()
            .filter(|sb| sb.state() == Estate::Running)
            .collect::<Vec<_>>()
            .len();

        let n = cmp::max(0i32, nb_workers as i32 - nb_running as i32) as usize;

        let min_build_level: u32 = {
            let mut min_build_level: Option<u32> = None;
            for sb in &mut self.songs_and_books {
                if sb.state() == Estate::Pending || sb.state() == Estate::Running {
                    min_build_level = match min_build_level {
                        None => Some(sb.build_level()),
                        Some(n) => Some(cmp::min(n, sb.build_level())),
                    }
                }
            }
            match min_build_level {
                None => return vec![],
                Some(n) => n,
            }
        };
        log::debug!(
            "{}:{} min build level is {}",
            file!(),
            line!(),
            min_build_level
        );

        let mut sb_to_start: Vec<String> = vec![];

        let mut count = 0;
        for sb in &mut self.songs_and_books {
            if count < n && sb.state() == Estate::Pending && sb.build_level() == min_build_level {
                sb.change_state(Estate::Running);
                sb_to_start.push(sb.id());
                log::info!("{}:{} push {}", file!(), line!(), sb.id());
                count += 1;
            }
        }

        sb_to_start
    }

    pub fn get(&self, id: String) -> Result<&Box<dyn TEstate>, Box<dyn std::error::Error>> {
        for sb in &self.songs_and_books {
            if sb.id() == id {
                return Ok(sb);
            }
        }
        Err(format!("no song or book for id '{}'", id).into())
    }

    pub fn get_mut(
        &mut self,
        id: String,
    ) -> Result<&mut Box<dyn TEstate>, Box<dyn std::error::Error>> {
        for sb in &mut self.songs_and_books {
            if sb.id() == id {
                return Ok(sb);
            }
        }
        Err(format!("no song or book for id '{id}'").into())
    }

    pub fn logsdone(&self) -> Vec<ListItem> {
        let mut logs = self
            .songs_and_books
            .iter()
            .filter_map(|sb| {
                match sb.uicontent() {
                    None => None,
                    Some(content) => {
                        let c = content;
                        let li = ListItem::new(c);
                        Some(li)
                        // self.logsdone.push(li);
                    }
                }
            })
            .collect::<Vec<_>>();

        let success_count = self
            .songs_and_books
            .iter()
            .filter(|sb| sb.state() == Estate::Success)
            .collect::<Vec<_>>()
            .len();
        let failure_count = self
            .songs_and_books
            .iter()
            .filter(|sb| sb.state() == Estate::Failure)
            .collect::<Vec<_>>()
            .len();
        let running_count = self
            .songs_and_books
            .iter()
            .filter(|sb| sb.state() == Estate::Running)
            .collect::<Vec<_>>()
            .len();
        let pending_count = self
            .songs_and_books
            .iter()
            .filter(|sb| sb.state() == Estate::Pending)
            .collect::<Vec<_>>()
            .len();

        let content = vec![Spans::from(vec![
            // Span::styled(format!("{}", BEAR), running_style),
            Span::raw(format!(
                "{} Success, {} Failures, {} Running, {} Pending ",
                success_count, failure_count, running_count, pending_count
            )),
        ])];
        let li = ListItem::new(content);
        logs.insert(0, li);
        logs
    }

    pub fn logsrunning(&self) -> &Vec<ListItem<'a>> {
        &self.logsrunning
    }

    pub fn move_state(
        &mut self,
        id: String,
        state: Estate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sb = self.get_mut(id)?;
        sb.change_state(state);
        Ok(())
    }

    pub fn handle_item(&mut self, s: LogItem, _f: &mut File) {
        let style_song = |s: &LogItemSong, label: String| {
            vec![Spans::from(vec![
                // Span::styled(format!("{}", emoji), Style::default()),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled(s.author.clone(), Style::default().fg(Color::Blue)),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(s.title.clone(), Style::default().fg(Color::Green)),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(label, Style::default().fg(Color::Magenta)),
            ])]
        };

        let style_book = |s: &LogItemBook, label: String| {
            vec![Spans::from(vec![
                // Span::styled(format!("{}", emoji), Style::default()),
                Span::raw(" "), // Span::styled(title, title_style),
                Span::styled("BOOK", Style::default().fg(Color::Blue)),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(s.title.clone(), Style::default().fg(Color::Green)),
                Span::raw(" @ "), // Span::styled(title, title_style),
                Span::styled(label, Style::default().fg(Color::Magenta)),
            ])]
        };

        let style = match s {
            LogItem::Song(s) => {
                let id = format!("{} @ {}", s.author, s.title);
                let sb = self.get_mut(id).unwrap();

                match &s.status {
                    ELogType::Started => style_song(&s, "start".to_string()),
                    ELogType::Lilypond(ly) => style_song(&s, format!("lilypond {ly}")),
                    ELogType::Lualatex(n) => style_song(&s, format!("lualatex #{n}")),
                    ELogType::Ps2pdf => {
                        sb.change_state(Estate::Running);
                        style_song(&s, "ps2pdf".to_string())
                    }
                    ELogType::Success | ELogType::NoNeedSuccess => {
                        sb.change_state(Estate::Success);
                        style_song(&s, "success".to_string())
                    }
                    ELogType::Failed | ELogType::NoNeedFailed => {
                        sb.change_state(Estate::Failure);
                        style_song(&s, "failed".to_string())
                    }
                }
            }
            LogItem::Book(s) => {
                let id = format!("BOOK @ {}", s.title);
                let sb = self.get_mut(id).unwrap();
                match &s.status {
                    ELogType::Started => style_book(&s, "started".to_string()),
                    ELogType::Lilypond(_) => style_book(&s, "lilypond".to_string()),
                    ELogType::Lualatex(n) => style_book(&s, format!("lualatex #{n}")),
                    ELogType::Ps2pdf => {
                        sb.change_state(Estate::Running);
                        style_book(&s, "ps2pdf".to_string())
                    }
                    ELogType::Success | ELogType::NoNeedSuccess => {
                        sb.change_state(Estate::Success);
                        style_book(&s, "success".to_string())
                    }
                    ELogType::Failed | ELogType::NoNeedFailed => {
                        sb.change_state(Estate::Failure);
                        style_book(&s, "failed".to_string())
                    }
                }
            }
        };

        self.logsrunning.insert(0, ListItem::new(style));
    }
}
