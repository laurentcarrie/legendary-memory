use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub enum UserStructureItemContent {
    Chords(Vec<String>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserStructureItem {
    pub texname: String,
    pub sectiontype: String,
    pub content: UserStructureItemContent,
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
    pub structure: Option<Vec<UserStructureItem>>,
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
