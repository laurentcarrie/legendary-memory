//! definition of the user input model ( the song.json and books ) in the user source tree
//!
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
/// a chord section is a section with an associated grid of chords
pub struct UserChordSection {
    /// the title of the section, must be a valid latex expression
    /// this will be rendered
    #[serde(rename(serialize = "title", deserialize = "title"))]
    pub section_title: String,
    /// the type of the section, ie couplet, refrain, ...
    /// must be a valid section type, see @todo show how to do it
    /// to add a section type, [edit this file](/legendary-memory/others/texfiles/sections.json)
    /// and rebuild the tool
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub section_type: String,
    /// a latex string, that will be shown in the grid rendering
    pub section_body: Option<String>,
    /// the chord rows of the section, each row is string, such as :
    /// `| A |B | Cm | Dm`
    /// `|` is the bar separator
    /// each bar has either one or two chords, a pause, or a repeat symbol @todo : show the list of valid symbols
    pub rows: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
/// a ref section is a section that refers to another one
/// for instance, you may have in a song ``verse 1`` which is a chord section, and ``verse 2``
/// that is a ref to ``verse 1``, because it has the same chords, and you don't want to show that again
pub struct UserRef {
    /// the title of the section, must be a valid latex expression
    /// this will be rendered
    #[serde(rename(serialize = "title", deserialize = "title"))]
    pub section_title: String,
    /// a latex string, that will be shown in the grid rendering
    pub section_body: Option<String>,
    /// the section type, that must be valid
    /// if missing, it will be the type of the section that is refered to
    /// to add a section type, [edit this file](/legendary-memory/others/texfiles/sections.json)
    /// and rebuild the tool
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub section_type: Option<String>,
    /// the id of the refered to section, it must exist
    pub link: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
/// the sum type for an item
pub enum UserStructureItemContent {
    /// a chord section
    Chords(UserChordSection),
    /// a reference to a chord section ( will be rendered as a reference )
    Ref(UserRef),
    /// an horizontal rule, with the width in page %, @todo implement that
    HRule(Option<u32>),
    NewColumn,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserBarComment {
    pub bar_number: u32,
    pub comment: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserStructureItem {
    /// the id of the item, that should be a valid latex variable, so no `_`, no space, no digit
    pub id: String,
    /// the item, this is a sum type
    pub item: UserStructureItemContent,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserSongInfo {
    /// the title of the song
    pub title: String,
    /// the author of the song
    pub author: String,
    pub tempo: u32,
    pub time_signature: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserSongMeta {
    /// the last modified date of the song.
    /// it is updated with a shell helper @todo : which one ?
    pub date: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserSongFiles {
    /// the list of additional texfiles, in the song directory
    pub tex: Vec<String>,
    /// the list of additional lilypond files, in the song directory
    /// this will be added in the OMakefile, and therefore virtually mounted in the build directory
    /// for each of this file, for example ```solo.ly```, at build time ```solo.output``` directory will be built
    /// in your source tex file, use
    /// ```\subimport{solo.output/}{solo}```
    /// to input the musicsheet in your output
    pub lilypond: Vec<String>,
    /// the list of additional wav files, to build.
    /// the wav file will be built from the lilypond file with the same name, that must exist
    pub wav: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
/// the ``song.json`` master file that defines a song
pub struct UserSong {
    pub info: UserSongInfo,
    /// the structure of the song : this is a list of items
    pub structure: Vec<UserStructureItem>,
    pub meta: UserSongMeta,
    pub files: UserSongFiles,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
/// a song in a book, this is the (author,title) tuple, that must exist
/// in the songdir tree, there has to be a ``song.json`` that has this
/// author and this title
pub struct UserBookSong {
    /// the author of the song, in the ``song.json`` file
    pub author: String,
    /// the title of the song, in the ``song.json`` file
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
/// a book, this means a collection of songs, with a title
pub struct UserBook {
    /// the title of the book, will be the name of the pdf file
    pub title: String,
    /// the list of songs in the book
    pub songs: Vec<UserBookSong>,
    pub lyrics_only: bool,
    pub cover_image: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserSongWithPath {
    pub song: UserSong,
    pub path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserBookWithPath {
    pub book: UserBook,
    pub path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserWorld {
    pub songs: Vec<UserSongWithPath>,
    pub books: Vec<UserBookWithPath>,
}
