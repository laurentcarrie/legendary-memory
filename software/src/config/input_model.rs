use crate::config::model::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub enum UserStructureItemContent {
    Chords(Vec<String>),
    Ref(String),
    HRule(Option<u32>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub struct UserStructureItem {
    pub title: String,
    pub section_id: String,
    pub sectiontype: String,
    pub item: UserStructureItemContent,
    pub text: String,
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
