use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Bar {
    pub chords: Vec<String>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Row {
    pub bars: Vec<Bar>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserSection {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Section {
    pub name: String,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserSong {
    pub cell_height: i32,
    pub cell_width: i32,
    pub title: String,
    pub author: String,
    pub texfiles: Vec<String>,
    pub lilypondfiles: Vec<String>,
    pub sections: Vec<UserSection>,
    pub outputtemplate: Option<String>,
    pub outputformat: Option<String>,
    pub chord_glyph_scale: Option<i32>,
    pub section_spacing: Option<i32>,
    pub wavfiles: Vec<String>,
    pub date: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserBookSong {
    pub author: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserBook {
    pub title: String,
    pub songs: Vec<UserBookSong>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Song {
    pub cell_height: i32,
    pub cell_width: i32,
    pub title: String,
    pub author: String,
    pub texfiles: Vec<String>,
    pub builddir: PathBuf,
    pub lilypondfiles: Vec<String>,
    pub sections: Vec<Section>,
    pub outputtemplate: String,
    pub outputformat: String,
    pub chord_glyph_scale: i32,
    pub section_spacing: i32,
    pub wavfiles: Vec<String>,
    pub date: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BookSong {
    pub author: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Book {
    pub title: String,
    pub songs: Vec<BookSong>,
    pub builddir: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct World {
    pub builddir: PathBuf,
    pub srcdir: PathBuf,
    pub songs: Vec<Song>,
    pub books: Vec<Book>,
}
