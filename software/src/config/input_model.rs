use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserChordSection {
    pub section_title: String,
    pub section_type: String,
    pub rows: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserRef {
    pub section_title: String,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserSong {
    pub title: String,
    pub author: String,
    pub texfiles: Vec<String>,
    pub lilypondfiles: Vec<String>,
    pub wavfiles: Vec<String>,
    pub date: String,
    pub structure: Vec<UserStructureItem>,
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
