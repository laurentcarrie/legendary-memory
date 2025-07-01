use crate::actions::build_pdf::{wrapped_build_pdf_book, wrapped_build_pdf_song};
use console::Emoji;
use human_sort::compare;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
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

use std::cmp::max;
use std::collections::HashSet;

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
    fn state(&self) -> Estate;
    fn build_level(&self) -> u32;
    fn change_state(&mut self, state: Estate) -> ();
    fn uicontent(&self) -> Option<Vec<Spans<'_>>>;
    fn build_pdf(
        &self,
        set: &mut JoinSet<()>,
        tx: Sender<LogItem>,
        world: &World,
        force_rebuild: bool,
    ) -> ();
}

impl TEstate for UISong {
    /// the current building state of a book or a song pdf
    fn state(&self) -> Estate {
        self.state.clone()
    }
    fn build_level(&self) -> u32 {
        0
    }
    fn change_state(&mut self, state: Estate) -> () {
        self.state = state
    }
    /// how to display
    fn uicontent(&self) -> Option<Vec<Spans<'_>>> {
        match self.state {
            Estate::Success => None,
            Estate::Pending | Estate::Failure | Estate::Running => {
                let content = vec![Spans::from(vec![
                    Span::styled(format!("{}", BEAR), Style::default().bg(Color::White)),
                    Span::raw(" "), // Span::styled(title, title_style),
                    Span::styled(self.song.author.clone(), Style::default().fg(Color::Blue)),
                    Span::raw(" @ "), // Span::styled(title, title_style),
                    Span::styled(self.song.title.clone(), Style::default().fg(Color::Green)),
                ])];
                Some(content)
                // let li = ListItem::new(content);
                // self.logsdone.push(li);
            }
        }
    }

    fn build_pdf(
        &self,
        set: &mut JoinSet<()>,
        tx: Sender<LogItem>,
        world: &World,
        force_rebuild: bool,
    ) -> () {
        let _ = set.spawn(wrapped_build_pdf_song(
            tx.clone(),
            world.clone(),
            self.song.clone(),
            force_rebuild,
        ));
    }
}

impl TEstate for UIBook {
    fn state(&self) -> Estate {
        self.state.clone()
    }
    fn build_level(&self) -> u32 {
        1
    }
    fn change_state(&mut self, state: Estate) -> () {
        self.state = state
    }
    fn uicontent(&self) -> Option<Vec<Spans<'_>>> {
        match self.state {
            Estate::Success => None,
            Estate::Pending | Estate::Failure | Estate::Running => {
                let content = vec![Spans::from(vec![
                    Span::styled(format!("{}", BEAR), Style::default().bg(Color::White)),
                    Span::raw(" "), // Span::styled(title, title_style),
                    Span::styled("BOOK", Style::default().fg(Color::Blue)),
                    Span::raw(" @ "), // Span::styled(title, title_style),
                    Span::styled(self.book.title.clone(), Style::default().fg(Color::Green)),
                ])];
                Some(content)
                // let li = ListItem::new(content);
                // self.logsdone.push(li);
            }
        }
    }

    fn build_pdf(
        &self,
        set: &mut JoinSet<()>,
        tx: Sender<LogItem>,
        world: &World,
        _force_rebuild: bool,
    ) -> () {
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
    logsdone: Vec<ListItem<'a>>,
    logsrunning: Vec<ListItem<'a>>,
    songs_and_books: Vec<Box<dyn TEstate>>,
}

impl<'a> UiModel<'a> {
    pub fn new(world: &World) -> UiModel<'a> {
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
            logsdone: vec![],
            logsrunning: vec![],
            songs_and_books: sb,
        }
    }

    pub fn init(&mut self) -> () {
        for sb in &mut self.songs_and_books {
            sb.change_state(Estate::Pending);
        }
    }

    /// if number of running builds < nb_workers, take n from pending and return them.
    /// as long as all songs are not built, ignore the books
    pub fn run_n_songs_or_books(&mut self, nb_workers: u32) -> Vec<&Box<dyn TEstate>> {
        let nb_running = self
            .songs_and_books
            .iter()
            .filter(|sb| sb.state() == Estate::Running)
            .collect::<Vec<_>>()
            .len();

        let n = max(0i32, nb_workers as i32 - nb_running as i32) as usize;

        let mut sb_to_start: Vec<&Box<dyn TEstate>> = vec![];

        let mut count = 0;
        for sb in &mut self.songs_and_books {
            if count < n && sb.state() == Estate::Pending {
                sb.change_state(Estate::Running);
                sb_to_start.push(sb.clone());
                count += 1;
            }
        }

        //@ todo : gerer le build level

        sb_to_start.clone()
    }

    pub fn make_logs(&mut self) -> () {
        let _author_style = Style::default().fg(Color::Blue);
        // let book_style = Style::default().fg(Color::Yellow).bg(Color::White);
        let _title_style = Style::default().fg(Color::Green);
        // let running_style = Style::default().fg(Color::White);

        let _map_format = |status: &Estate| -> Span<'_> {
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

        for sb in self.songs_and_books.iter() {
            match sb.uicontent() {
                None => (),
                Some(content) => {
                    let c = content.clone();
                    let _li = ListItem::new(c);
                    // self.logsdone.push(li);
                }
            }
        }

        // for (book, status) in self.books.iter() {
        //     match status {
        //         Estate::Success => (),
        //         Estate::Pending | Estate::Failure | Estate::Running => {
        //             let content = vec![Spans::from(vec![
        //                 map_format(status),
        //                 Span::raw(" "), // Span::styled(title, title_style),
        //                 Span::styled("BOOK", author_style),
        //                 Span::raw(" @ "), // Span::styled(title, title_style),
        //                 Span::styled(book.title.clone(), title_style),
        //             ])];
        //             let li = ListItem::new(content);
        //             self.logsdone.push(li);
        //         }
        //     }
        // }

        // let pending_count = self
        //     .songs
        //     .iter()
        //     .filter(|(_, status)| status == &Estate::Pending)
        //     .collect::<Vec<_>>()
        //     .len()
        //     + self
        //         .books
        //         .iter()
        //         .filter(|(_, status)| status == &Estate::Pending)
        //         .collect::<Vec<_>>()
        //         .len();
        // let running_count = self
        //     .songs
        //     .iter()
        //     .filter(|(_, status)| status == &Estate::Running)
        //     .collect::<Vec<_>>()
        //     .len()
        //     + self
        //         .books
        //         .iter()
        //         .filter(|(_, status)| status == &Estate::Running)
        //         .collect::<Vec<_>>()
        //         .len();
        // let success_count = self
        //     .songs
        //     .iter()
        //     .filter(|(_, status)| status == &Estate::Success)
        //     .collect::<Vec<_>>()
        //     .len()
        //     + self
        //         .books
        //         .iter()
        //         .filter(|(_, status)| status == &Estate::Success)
        //         .collect::<Vec<_>>()
        //         .len();
        // let failure_count = self
        //     .songs
        //     .iter()
        //     .filter(|(_, status)| status == &Estate::Failure)
        //     .collect::<Vec<_>>()
        //     .len()
        //     + self
        //         .books
        //         .iter()
        //         .filter(|(_, status)| status == &Estate::Failure)
        //         .collect::<Vec<_>>()
        //         .len();

        // let content = vec![Spans::from(vec![
        //     // Span::styled(format!("{}", BEAR), running_style),
        //     Span::raw(format!(
        //         "{} Success, {} Failures, {} Running, {} Pending ",
        //         success_count, failure_count, running_count, pending_count
        //     )),
        // ])];
        // let li = ListItem::new(content);
        // self.logsdone.insert(0, li);
        ()
        // }
    }

    // pub fn move_song_state(&mut self, log: &LogItemSong, state: Estate) {
    //     self.songs = self
    //         .songs
    //         .clone()
    //         .iter()
    //         .map(|(song, current_state)| {
    //             if song.author == log.author && song.title == log.title {
    //                 (song.clone(), state.clone())
    //             } else {
    //                 (song.clone(), current_state.clone())
    //             }
    //         })
    //         .collect::<Vec<_>>()
    // }

    // pub fn move_book_state(&mut self, log: &LogItemBook, state: Estate) {
    //     self.books = self
    //         .books
    //         .clone()
    //         .iter()
    //         .map(|(book, current_state)| {
    //             if book.title == log.title {
    //                 (book.clone(), state.clone())
    //             } else {
    //                 (book.clone(), current_state.clone())
    //             }
    //         })
    //         .collect::<Vec<_>>()
    // }

    pub fn handle_item(&mut self, _s: LogItem, _f: &mut File) -> () {
        // let success_style = Style::default().bg(Color::Green);
        // let failure_style = Style::default().bg(Color::Red);
        // let author_style = Style::default().fg(Color::Blue);
        // let book_style = Style::default().fg(Color::Yellow).bg(Color::White);
        // let title_style = Style::default().fg(Color::Green);
        // let lily_style = Style::default().fg(Color::Cyan);
        // let no_need_success_style = Style::default().fg(Color::LightGreen);
        // let no_need_failed_style = Style::default().fg(Color::LightRed);
        // let thread_style = Style::default().fg(Color::Cyan).bg(Color::White);
        // match &s {
        //     LogItem::Song(s) => {
        //         writeln!(f, "{:?}", &s.clone()).unwrap();
        //         match &s.status {
        //             ELogType::Started => {
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(format!("[  STARTED  ] ")), // Span::styled(title, title_style),
        //                     Span::styled(s.author.clone(), author_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Lualatex(count) => {
        //                 let ss = format!("[lualatex #{count}] ").clone();
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(ss), // Span::styled(title, title_style),
        //                     Span::styled(s.author.clone(), author_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Ps2pdf => {
        //                 let ss = format!("[   ps2pdf  ] ").clone();
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(ss), // Span::styled(title, title_style),
        //                     Span::styled(s.author.clone(), author_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Lilypond(ref lyfile) => {
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(format!("[  lilypond ] ")), // Span::styled(title, title_style),
        //                     Span::styled(s.author.clone(), author_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                     Span::raw(" ; "), // Span::styled(title, title_style),
        //                     // Span::raw(format!(" / {lyfile.clone()}").as_str()), // Span::styled(title, title_style),
        //                     Span::styled(lyfile.clone(), lily_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Success => {
        //                 self.move_song_state(s, Estate::Success);
        //                 // self.success_keys
        //                 //     .insert((s.author.clone(), s.title.clone()));
        //                 // self.running_keys
        //                 //     .remove(&(s.author.clone(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("   {}   ", SPARKLE), success_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled(s.author.clone(), author_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //             ELogType::Failed => {
        //                 self.move_song_state(s, Estate::Failure);

        //                 // self.failure_keys
        //                 //     .insert((s.author.clone(), s.title.clone()));
        //                 // self.running_keys
        //                 //     .remove(&(s.author.clone(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("{}", SPARKLE), failure_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled(s.author.clone(), author_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //             ELogType::NoNeedSuccess => {
        //                 self.move_song_state(s, Estate::Success);
        //                 // self.success_keys
        //                 //     .insert((s.author.clone(), s.title.clone()));
        //                 // self.running_keys
        //                 //     .remove(&(s.author.clone(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("{}", ZZZ), no_need_success_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled(s.author.clone(), author_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //             ELogType::NoNeedFailed => {
        //                 self.move_song_state(s, Estate::Failure);
        //                 // self.success_keys
        //                 //     .insert((s.author.clone(), s.title.clone()));
        //                 // self.running_keys
        //                 //     .remove(&(s.author.clone(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("{}", ZZZ), no_need_failed_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled(s.author.clone(), author_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //         }
        //     }
        //     LogItem::Book(s) => {
        //         writeln!(f, "{:?}", &s.clone()).unwrap();
        //         match &s.status {
        //             ELogType::Started => {
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(format!("[  STARTED  ] ")), // Span::styled(title, title_style),
        //                     Span::styled("BOOK", book_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Lualatex(count) => {
        //                 let ss = format!("[lualatex #{count}] ").clone();
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(ss), // Span::styled(title, title_style),
        //                     Span::styled("BOOK", book_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Ps2pdf => {
        //                 let ss = format!("[   ps2pdf  ] ").clone();
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(ss), // Span::styled(title, title_style),
        //                     Span::styled("BOOK", book_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Lilypond(ref lyfile) => {
        //                 let content = vec![Spans::from(vec![
        //                     Span::raw(format!("[  lilypond ] ")), // Span::styled(title, title_style),
        //                     Span::styled("BOOK", book_style),
        //                     Span::raw(" @ "), // Span::styled(title, title_style),
        //                     Span::styled(s.title.clone(), title_style),
        //                     Span::raw(" ; "), // Span::styled(title, title_style),
        //                     // Span::raw(format!(" / {lyfile.clone()}").as_str()), // Span::styled(title, title_style),
        //                     Span::styled(lyfile.clone(), lily_style),
        //                 ])];
        //                 let li = ListItem::new(content);
        //                 self.logsrunning.insert(0, li);
        //             }
        //             ELogType::Success => {
        //                 self.move_book_state(s, Estate::Success);
        //                 // self.success_keys
        //                 //     .insert(("BOOK".to_string(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("   {}   ", SPARKLE), success_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled("BOOK", book_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //             ELogType::Failed => {
        //                 self.move_book_state(s, Estate::Failure);
        //                 // self.failure_keys
        //                 //     .insert(("BOOK".to_string(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("{}", SPARKLE), failure_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled("BOOK", book_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //             ELogType::NoNeedSuccess => {
        //                 self.move_book_state(s, Estate::Success);
        //                 // self.success_keys
        //                 //     .insert(("BOOK".to_string(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("{}", ZZZ), no_need_success_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled("BOOK", book_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //             ELogType::NoNeedFailed => {
        //                 self.move_book_state(s, Estate::Success);
        //                 // self.success_keys
        //                 //     .insert(("BOOK".to_string(), s.title.clone()));
        //                 {
        //                     let content = vec![Spans::from(vec![
        //                         Span::styled(format!("{}", ZZZ), no_need_failed_style),
        //                         Span::raw(" "), // Span::styled(title, title_style),
        //                         Span::styled("BOOK", book_style),
        //                         Span::raw(" @ "), // Span::styled(title, title_style),
        //                         Span::styled(s.title.clone(), title_style),
        //                     ])];
        //                     let li = ListItem::new(content);
        //                     self.logsrunning.insert(0, li);
        //                 }
        //             }
        //         }
        //     }
        // };
    }

    pub fn logsrunning(&mut self) -> Vec<ListItem<'a>> {
        self.logsrunning.clone()
    }

    pub fn logsdone(&mut self) -> Vec<ListItem<'a>> {
        self.logsdone.clone()
    }
}
