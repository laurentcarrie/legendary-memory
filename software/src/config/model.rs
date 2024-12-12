use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

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
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct Row {
    pub bar_number: u32,
    pub bars: Vec<Bar>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Chords {
    pub section_title: String,
    pub section_id: String,
    pub section_type: String,
    pub bar_number: u32,
    pub nb_bars: u32,
    pub nbcols: u32,
    pub nbrows: u32,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Ref {
    pub bar_number: u32,
    pub nb_bars: u32,
    pub section_title: String,
    pub section_id: String,
    pub section_type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub enum StructureItemContent {
    ItemChords(Chords),
    ItemRef(Ref),
    ItemHRule(HRule),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct StructureItem {
    pub item: StructureItemContent,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Song {
    pub title: String,
    pub author: String,
    pub pdfname: String,
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
    pub sections: BTreeMap<String, Section>,
}
