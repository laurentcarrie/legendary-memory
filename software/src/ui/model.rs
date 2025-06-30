use console::Emoji;
use human_sort::compare;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
// static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
// static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static ZZZ: Emoji<'_, '_> = Emoji("💤", "zzz");
static BEAR: Emoji<'_, '_> = Emoji("🐻", "bear");

use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;

use std::cmp::max;
use std::collections::HashSet;

use crate::model::model::{Book, ELogType, LogItem, LogItemBook, LogItemSong, Song, World};

#[derive(PartialEq, Clone)]
pub enum Estate {
    Pending,
    Running,
    Success,
    Failure,
}

pub struct UISong {
    song:Song,
    state:Estate
}

pub struct UIBook {
    book:Book,
    state:Estate
}

pub trait TEstate {
    fn state(&self) -> EState;
}


pub struct UiModel<'a> {
    //  set: JoinSet<()>,
    songs: Vec<(Song, Estate)>,
    books: Vec<(Book, Estate)>,
    logsdone: Vec<ListItem<'a>>,
    logsrunning: Vec<ListItem<'a>>,
}

impl<'a> UiModel<'a> {
    pub fn new(world: &World) -> UiModel<'a> {
        let mut sorted_songs = world
            .songs
            .clone()
            .iter()
            .map(|song| (song.clone(), Estate::Pending))
            .collect::<Vec<_>>();
        sorted_songs.sort_by(
            |a, b| match compare(a.0.author.as_str(), b.0.author.as_str()) {
                Ordering::Equal => compare(a.0.title.as_str(), b.0.title.as_str()),
                x => x,
            },
        );

        let mut sorted_books = world
            .books
            .clone()
            .iter()
            .map(|book| (book.clone(), Estate::Pending))
            .collect::<Vec<_>>();
        sorted_books.sort_by(|a, b| compare(a.0.title.as_str(), b.0.title.as_str()));

        UiModel {
            songs: sorted_songs.clone(),
            books: sorted_books.clone(),
            // set: JoinSet::new(),
            logsdone: vec![],
            logsrunning: vec![],
        }
    }

    pub fn init(&mut self) -> () {
        self.songs = self
            .songs
            .clone()
            .iter()
            .clone()
            .map(|(song, _)| (song.clone(), Estate::Pending))
            .collect::<Vec<_>>();
        self.books = self
            .books
            .iter()
            .map(|(book, _)| (book.clone(), Estate::Pending))
            .collect::<Vec<_>>();
    }

    /// if number of running builds < nb_workers, take n from pending and return them.
    /// as long as all songs are not built, ignore the books
    pub fn run_n_songs_or_books(&mut self, nb_workers: u32) -> (Vec<Song>, Vec<Book>) {
        let nb_running = self
            .songs
            .iter()
            .filter(|(_, state)| state == &Estate::Running)
            .collect::<Vec<_>>()
            .len()
            + self
                .books
                .iter()
                .filter(|(_, state)| state == &Estate::Running)
                .collect::<Vec<_>>()
                .len();
        let n = max(0i32, nb_workers as i32 - nb_running as i32);

        let mut songs_to_start: Vec<Song> = vec![];
        (self.songs) = {
            let mut count = 0;
            self.songs
                .iter()
                .map(|(song, state)| {
                    if state == &Estate::Pending && count < n {
                        count += 1;
                        songs_to_start.push(song.clone());
                        (song.clone(), Estate::Running)
                    } else {
                        (song.clone(), state.clone())
                    }
                })
                .collect::<Vec<_>>()
        };

        let all_songs_done = self
            .songs
            .iter()
            .filter(|(_, state)| state == &Estate::Success)
            .collect::<Vec<_>>()
            .len()
            == self.songs.len();

        let mut books_to_start: Vec<Book> = vec![];
        if all_songs_done {
            self.books = {
                let mut count = 0;
                self.books
                    .iter()
                    .map(|(book, state)| {
                        if state == &Estate::Pending && count < n {
                            count += 1;
                            books_to_start.push(book.clone());
                            (book.clone(), Estate::Running)
                        } else {
                            (book.clone(), state.clone())
                        }
                    })
                    .collect::<Vec<_>>()
            }
        };
        (songs_to_start.clone(), books_to_start.clone())
    }

    pub fn make_logs(&mut self) -> () {
        let author_style = Style::default().fg(Color::Blue);
        // let book_style = Style::default().fg(Color::Yellow).bg(Color::White);
        let title_style = Style::default().fg(Color::Green);
        // let running_style = Style::default().fg(Color::White);

        let map_format = |status: &Estate| -> Span<'_> {
            match status {
                Estate::Pending => {
                    Span::styled(format!("{}", BEAR), Style::default().bg(Color::White))
                }
                Estate::Running => {
                    Span::styled(format!("{}", TRUCK), Style::default().bg(Color::Blue))
                }
                Estate::Success => {
                    Span::styled(format!("{}", SPARKLE), Style::default().bg(Color::Green))
                }
                Estate::Failure => {
                    Span::styled(format!("{}", SPARKLE), Style::default().bg(Color::Red))
                }
            }
        };
        self.logsdone.clear();

        for (song, status) in self.songs.iter() {
            match status {
                Estate::Success => (),
                Estate::Pending | Estate::Failure | Estate::Running => {
                    let content = vec![Spans::from(vec![
                        map_format(status),
                        Span::raw(" "), // Span::styled(title, title_style),
                        Span::styled(song.author.clone(), author_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(song.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    self.logsdone.push(li);
                }
            }
        }

        for (book, status) in self.books.iter() {
            match status {
                Estate::Success => (),
                Estate::Pending | Estate::Failure | Estate::Running => {
                    let content = vec![Spans::from(vec![
                        map_format(status),
                        Span::raw(" "), // Span::styled(title, title_style),
                        Span::styled("BOOK", author_style),
                        Span::raw(" @ "), // Span::styled(title, title_style),
                        Span::styled(book.title.clone(), title_style),
                    ])];
                    let li = ListItem::new(content);
                    self.logsdone.push(li);
                }
            }
        }


        let pending_count = self
            .songs
            .iter()
            .filter(|(_, status)| status == &Estate::Pending)
            .collect::<Vec<_>>()
            .len()
            + self
                .books
                .iter()
                .filter(|(_, status)| status == &Estate::Pending)
                .collect::<Vec<_>>()
                .len();
        let running_count = self
            .songs
            .iter()
            .filter(|(_, status)| status == &Estate::Running)
            .collect::<Vec<_>>()
            .len()
            + self
                .books
                .iter()
                .filter(|(_, status)| status == &Estate::Running)
                .collect::<Vec<_>>()
                .len();
        let success_count = self
            .songs
            .iter()
            .filter(|(_, status)| status == &Estate::Success)
            .collect::<Vec<_>>()
            .len()
            + self
                .books
                .iter()
                .filter(|(_, status)| status == &Estate::Success)
                .collect::<Vec<_>>()
                .len();
        let failure_count = self
            .songs
            .iter()
            .filter(|(_, status)| status == &Estate::Failure)
            .collect::<Vec<_>>()
            .len()
            + self
                .books
                .iter()
                .filter(|(_, status)| status == &Estate::Failure)
                .collect::<Vec<_>>()
                .len();

        let content = vec![Spans::from(vec![
            // Span::styled(format!("{}", BEAR), running_style),
            Span::raw(format!(
                "{} Success, {} Failures, {} Running, {} Pending ",
                success_count, failure_count, running_count, pending_count
            )), // Span::styled(title, title_style),
                // Span::styled(song.author.clone(), author_style),
                // Span::raw(" @ "), // Span::styled(title, title_style),
                // Span::styled(song.title.clone(), title_style),
        ])];
        let li = ListItem::new(content);
        self.logsdone.insert(0, li);
        ()
        // }
    }

    pub fn move_song_state(&mut self, log: &LogItemSong, state: Estate) {
        self.songs = self
            .songs
            .clone()
            .iter()
            .map(|(song, current_state)| {
                if song.author == log.author && song.title == log.title {
                    (song.clone(), state.clone())
                } else {
                    (song.clone(), current_state.clone())
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn move_book_state(&mut self, log: &LogItemBook, state: Estate) {
        self.books = self
            .books
            .clone()
            .iter()
            .map(|(book, current_state)| {
                if book.title == log.title {
                    (book.clone(), state.clone())
                } else {
                    (book.clone(), current_state.clone())
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn handle_item(&mut self, s: LogItem, f: &mut File) -> () {
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
                        self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
                    }
                    ELogType::Success => {
                        self.move_song_state(s, Estate::Success);
                        // self.success_keys
                        //     .insert((s.author.clone(), s.title.clone()));
                        // self.running_keys
                        //     .remove(&(s.author.clone(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("   {}   ", SPARKLE), success_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled(s.author.clone(), author_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                    ELogType::Failed => {
                        self.move_song_state(s, Estate::Failure);

                        // self.failure_keys
                        //     .insert((s.author.clone(), s.title.clone()));
                        // self.running_keys
                        //     .remove(&(s.author.clone(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("{}", SPARKLE), failure_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled(s.author.clone(), author_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                    ELogType::NoNeedSuccess => {
                        self.move_song_state(s, Estate::Success);
                        // self.success_keys
                        //     .insert((s.author.clone(), s.title.clone()));
                        // self.running_keys
                        //     .remove(&(s.author.clone(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("{}", ZZZ), no_need_success_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled(s.author.clone(), author_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                    ELogType::NoNeedFailed => {
                        self.move_song_state(s, Estate::Failure);
                        // self.success_keys
                        //     .insert((s.author.clone(), s.title.clone()));
                        // self.running_keys
                        //     .remove(&(s.author.clone(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("{}", ZZZ), no_need_failed_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled(s.author.clone(), author_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
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
                        self.logsrunning.insert(0, li);
                    }
                    ELogType::Success => {
                        self.move_book_state(s, Estate::Success);
                        // self.success_keys
                        //     .insert(("BOOK".to_string(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("   {}   ", SPARKLE), success_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled("BOOK", book_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                    ELogType::Failed => {
                        self.move_book_state(s, Estate::Failure);
                        // self.failure_keys
                        //     .insert(("BOOK".to_string(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("{}", SPARKLE), failure_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled("BOOK", book_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                    ELogType::NoNeedSuccess => {
                        self.move_book_state(s, Estate::Success);
                        // self.success_keys
                        //     .insert(("BOOK".to_string(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("{}", ZZZ), no_need_success_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled("BOOK", book_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                    ELogType::NoNeedFailed => {
                        self.move_book_state(s, Estate::Success);
                        // self.success_keys
                        //     .insert(("BOOK".to_string(), s.title.clone()));
                        {
                            let content = vec![Spans::from(vec![
                                Span::styled(format!("{}", ZZZ), no_need_failed_style),
                                Span::raw(" "), // Span::styled(title, title_style),
                                Span::styled("BOOK", book_style),
                                Span::raw(" @ "), // Span::styled(title, title_style),
                                Span::styled(s.title.clone(), title_style),
                            ])];
                            let li = ListItem::new(content);
                            self.logsrunning.insert(0, li);
                        }
                    }
                }
            }
        };
    }

    pub fn logsrunning(&mut self) -> Vec<ListItem<'a>> {
        self.logsrunning.clone()
    }

    pub fn logsdone(&mut self) -> Vec<ListItem<'a>> {
        self.logsdone.clone()
    }
}
