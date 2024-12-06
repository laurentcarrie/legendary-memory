use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct UserSection {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

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
