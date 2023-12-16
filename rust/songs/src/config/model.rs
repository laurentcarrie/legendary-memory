use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Bar {
    pub chords: Vec<String>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Row {
    pub bars: Vec<Bar>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserSection {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Section {
    pub name: String,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
pub struct World {
    pub builddir: PathBuf,
    pub srcdir: PathBuf,
    pub project_yaml_paths: Vec<PathBuf>,
    pub songs: Vec<Song>,
}
