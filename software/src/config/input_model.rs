use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserChordSection {
    pub section_title: String,
    pub section_type: String,
    pub section_body: Option<String>,
    pub rows: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserRef {
    pub section_title: String,
    pub section_body: Option<String>,
    pub link: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub enum UserStructureItemContent {
    Chords(UserChordSection),
    Ref(UserRef),
    HRule(Option<u32>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserStructureItem {
    pub id: String,
    pub item: UserStructureItemContent,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserSong {
    pub title: String,
    pub author: String,
    pub texfiles: Vec<String>,
    pub lilypondfiles: Vec<String>,
    pub wavfiles: Vec<String>,
    pub date: String,
    pub structure: Vec<UserStructureItem>,
    pub tempo: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserBookSong {
    pub author: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserBook {
    pub title: String,
    pub songs: Vec<UserBookSong>,
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

