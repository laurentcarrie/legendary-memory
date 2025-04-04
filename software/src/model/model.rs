use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct TimeSignature {
    pub top: u8,
    pub low: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Section {
    pub id: String,
    pub color: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct HRule {
    pub percent: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Bar {
    pub chords: Vec<String>,
    pub time_signature: Option<TimeSignature>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Row {
    pub row_start_bar_number: u32,
    pub bars: Vec<Bar>,
    pub repeat: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Chords {
    pub section_title: String,
    pub section_id: String,
    pub section_type: String,
    pub section_body: String,
    pub row_start_bar_number: u32,
    pub nb_bars: u32,
    pub nbcols: u32,
    pub nbrows: u32,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Ref {
    pub row_start_bar_number: u32,
    pub nb_bars: u32,
    pub section_title: String,
    pub section_id: String,
    pub section_type: String,
    pub section_body: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub enum StructureItemContent {
    ItemChords(Chords),
    ItemRef(Ref),
    ItemHRule(HRule),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct StructureItem {
    pub item: StructureItemContent,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Song {
    pub title: String,
    pub author: String,
    pub tempo: u32,
    pub time_signature: TimeSignature,
    pub pdfname: String,
    pub texfiles: Vec<String>,
    pub builddir: PathBuf,
    pub lilypondfiles: Vec<String>,
    pub wavfiles: Vec<String>,
    pub date: String,
    pub structure: Vec<StructureItem>,
    pub srcdir: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BookSong {
    pub author: String,
    pub title: String,
    pub pdfname: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Book {
    pub title: String,
    pub songs: Vec<BookSong>,
    pub builddir: PathBuf,
    pub pdfname: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct World {
    pub builddir: PathBuf,
    pub srcdir: PathBuf,
    pub songs: Vec<Song>,
    pub books: Vec<Book>,
    pub sections: BTreeMap<String, Section>,
    pub broken_songs: Vec<(PathBuf, String)>,
    pub broken_books: Vec<(PathBuf, String)>,
}
