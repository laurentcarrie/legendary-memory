use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Row {
    pub bar_number:u32,
    pub chords:Vec<String>
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub enum StructureItemContent {
    Chords(Vec<Row>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct StructureItem {
    pub texname: String,
    pub text: String,
    pub sectiontype: String,
    pub content: StructureItemContent,
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
