use handlebars::Handlebars;
use std::path::{Path, PathBuf};
use yamake::model::{Edge, ExpandError, ExpandResult, GNode, GRootNode};

use super::{PdfFile, TexFile};
use crate::helpers::register_helpers;
use crate::model::{Book, normalize_name};
use crate::settings::Settings;

const MAIN_BOOK_TEMPLATE: &str = include_str!("../resources/texfiles/mainbook.tex");
const PREAMBLE_TEMPLATE: &str = include_str!("../resources/texfiles/preamble.tex");
const TIKZ_SPLINE_LIB: &str = include_str!("../resources/texfiles/tikzlibraryspline.code.tex");
const SECTIONS_TEMPLATE: &str = include_str!("../resources/texfiles/sections.tex");
const CHORDS_TEX: &str = include_str!("../resources/texfiles/chords.tex");

/// Directory of the songs sandbox where the books are built, and where their
/// yaml files are copied from the books source directory.
pub const BOOKS_DIR: &str = "books";

/// One song of a book, resolved against the world.
#[derive(Debug, Clone)]
pub struct BookSong {
    /// Directory holding the song, relative to srcdir (and to the songs sandbox).
    pub dir: PathBuf,
    /// Song author, for the setlist page.
    pub author: String,
    /// Song title, for the setlist page.
    pub title: String,
}

/// Root node for a book yaml file that expands into the book build graph
/// (master TeX file and its inputs). The book PDF collates the `body.tex` of
/// every listed song, imported from the song's own build directory.
pub struct BookYml {
    /// Path to the book yaml file relative to srcdir (e.g. `books/rock.yml`).
    pub path: PathBuf,
    /// Parsed book data.
    pub book: Book,
    /// Songs of the book, in the order they are collated.
    pub songs: Vec<BookSong>,
}

/// A navigation word, linked to the song of the given index when there is one,
/// and greyed out when there is not: the first song has no predecessor and the
/// last no successor, and the header keeps the same shape either way.
fn nav_link(word: &str, target: Option<usize>) -> String {
    match target {
        Some(i) => format!("\\hyperref[book:song:{i}]{{\\textcolor{{blue}}{{{word}}}}}"),
        None => format!("\\textcolor{{gray}}{{{word}}}"),
    }
}

/// Escapes the characters that would otherwise be typeset as LaTeX markup.
fn tex_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' | '%' | '$' | '#' | '_' | '{' | '}' => {
                out.push('\\');
                out.push(c);
            }
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\textasciicircum{}"),
            '\\' => out.push_str("\\textbackslash{}"),
            _ => out.push(c),
        }
    }
    out
}

impl BookYml {
    /// Creates a new `BookYml` node from a book and its resolved songs.
    pub fn new(path: PathBuf, book: Book, songs: Vec<BookSong>) -> Self {
        Self { path, book, songs }
    }

    /// Directory where the book is built, relative to the songs sandbox.
    /// Named after the book, so two books never share a directory.
    pub fn builddir(&self) -> PathBuf {
        Path::new(BOOKS_DIR).join(normalize_name(&self.book.name))
    }

    /// Path of the book master TeX file, relative to the songs sandbox.
    pub fn tex_path(&self) -> PathBuf {
        self.builddir().join("main.tex")
    }

    /// Path of the book PDF, relative to the songs sandbox.
    pub fn pdf_path(&self) -> PathBuf {
        self.builddir().join("main.pdf")
    }

    /// Path of the delivered PDF, relative to the songs sandbox:
    /// `../pdf/book-<name>.pdf`.
    pub fn delivery_path(&self) -> PathBuf {
        Path::new("../pdf").join(format!("{}.pdf", self.book.file_stem_of_book()))
    }

    /// Prefix that walks back from the book build directory to the songs
    /// sandbox root, so a song directory can be imported from the book.
    fn back_to_root(&self) -> PathBuf {
        self.builddir()
            .components()
            .map(|_| "..")
            .collect::<PathBuf>()
    }
}

impl GRootNode for BookYml {
    fn tag(&self) -> String {
        "book.yml".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn expand(&self, sandbox: &Path, _predecessors: &[&(dyn GNode + Send + Sync)]) -> ExpandResult {
        let builddir = self.builddir();
        let builddir_full = sandbox.join(&builddir);
        std::fs::create_dir_all(&builddir_full)
            .map_err(|e| ExpandError::WriteError(builddir_full.clone(), e))?;

        let settings = Settings::load(sandbox).map_err(ExpandError::Other)?;

        // The setlist page, one row per song, and the body, which imports each
        // song from its own directory: \import keeps the \input commands inside
        // body.tex (song.tikz, lyrics/*.tex) relative to that directory.
        // Each setlist entry links to the page its song starts on: the anchor
        // is dropped right after the clearpage that opened that page, so the
        // link lands on the song rather than on the tail of the previous one.
        let back = self.back_to_root();
        let last = self.songs.len().saturating_sub(1);
        let mut setlist = String::new();
        let mut songs_body = String::new();
        for (i, song) in self.songs.iter().enumerate() {
            setlist.push_str(&format!(
                "\t\t\\item \\hyperref[book:song:{i}]{{{} / {}}}\n",
                tex_escape(&song.author),
                tex_escape(&song.title)
            ));

            // The navigation goes in the left of the footer, the one corner of
            // the page furniture nothing else uses. A line above the song would
            // cost body space and spill a song that fills its page onto
            // another, and the header is clipped: the top margin is 1cm, which
            // leaves it off the printable area. It is set per song, so every
            // page of a long song carries it, raised a line above the footer so
            // it clears the song title the footer centres there, which is as
            // wide as the title happens to be.
            let nav = format!(
                "\\lfoot{{\\raisebox{{2.6em}}[0pt][0pt]{{\\small {} \\quad {} \\quad {}}}}}\n",
                nav_link("précédent", (i > 0).then(|| i - 1)),
                "\\hyperref[book:top]{\\textcolor{blue}{table des matières}}",
                nav_link("suivant", (i < last).then(|| i + 1)),
            );

            let dir = back.join(&song.dir);
            let dir = format!("{}/", dir.display());
            songs_body.push_str(&format!(
                "{nav}\\phantomsection\\label{{book:song:{i}}}\n\\import{{{dir}}}{{data.tex}}\n\\import{{{dir}}}{{body.tex}}\n\\clearpage\n\n"
            ));
        }

        let mut handlebars = Handlebars::new();
        register_helpers(&mut handlebars);

        // The preamble is a song template: it declares the song macros that each
        // song's data.tex redefines as the book goes. Only the book name is of
        // any use here, as the title of a song that does not exist.
        let preamble_data = serde_json::json!({
            "song": {
                "info": {"title": self.book.name, "author": "", "tempo": ""},
                "meta": {"date": null},
            },
            "settings": settings,
        });
        let book_data = serde_json::json!({
            "book": {"name": self.book.name, "tags": self.book.tags},
            "setlist": setlist,
            "songs_body": songs_body,
            "settings": settings,
        });

        for (filename, template, data) in [
            ("main.tex", MAIN_BOOK_TEMPLATE, &book_data),
            ("preamble.tex", PREAMBLE_TEMPLATE, &preamble_data),
            ("sections.tex", SECTIONS_TEMPLATE, &book_data),
        ] {
            let content = handlebars.render_template(template, data).map_err(|e| {
                log::error!(
                    "Failed to render {filename} of book {}: {e}",
                    self.book.name
                );
                ExpandError::Other(e.to_string())
            })?;
            let full_path = builddir_full.join(filename);
            std::fs::write(&full_path, &content)
                .map_err(|e| ExpandError::WriteError(full_path, e))?;
        }

        // chords.tex and the spline library are static, and are written next to
        // the master file because lualatex runs in the book directory.
        for (filename, content) in [
            ("chords.tex", CHORDS_TEX),
            ("tikzlibraryspline.code.tex", TIKZ_SPLINE_LIB),
        ] {
            let full_path = builddir_full.join(filename);
            std::fs::write(&full_path, content)
                .map_err(|e| ExpandError::WriteError(full_path, e))?;
        }

        let tex_path = self.tex_path();
        let nodes: Vec<Box<dyn GNode + Send + Sync>> = vec![
            Box::new(TexFile::new(tex_path.clone())),
            Box::new(TexFile::new(builddir.join("preamble.tex"))),
            Box::new(TexFile::new(builddir.join("sections.tex"))),
            Box::new(TexFile::new(builddir.join("chords.tex"))),
            Box::new(TexFile::new(builddir.join("tikzlibraryspline.code.tex"))),
        ];

        // The PDF node is pre-added to the graph by make_all, as for songs.
        let edges: Vec<Edge> = vec![Edge {
            nfrom: Box::new(TexFile::new(tex_path)),
            nto: Box::new(PdfFile::new(self.pdf_path())),
        }];

        Ok((nodes, edges))
    }
}
