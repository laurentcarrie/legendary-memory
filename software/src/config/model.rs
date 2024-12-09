use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Row {
    pub bar_number: u32,
    pub chords: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Chords {
    pub section_id: String,
    pub sectiontype: String,
    pub bar_number: u32,
    pub nb_bars: u32,
    pub colspec: String,
    pub nbcols: u32,
    pub nbrows: u32,
    pub CodeBefore: String,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Ref {
    pub bar_number: u32,
    pub nb_bars: u32,
    pub section_id: String,
    pub sectiontype: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub enum StructureItemContent {
    ItemChords(Chords),
    ItemRef(Ref),
    ItemHRule(),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct StructureItem {
    pub title: String,
    pub text: String,
    pub item: StructureItemContent,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Song {
    pub title: String,
    pub author: String,
    pub texfiles: Vec<String>,
    pub builddir: PathBuf,
    pub lilypondfiles: Vec<String>,
    pub wavfiles: Vec<String>,
    pub date: String,
    pub structure: Vec<StructureItem>,
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
