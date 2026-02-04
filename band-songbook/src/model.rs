use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Song {
    pub files: SongFiles,
    pub info: SongInfo,
    pub meta: SongMeta,
    pub structure: Vec<StructureItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongFiles {
    #[serde(default)]
    pub lilypond: Vec<String>,
    #[serde(default)]
    pub tex: Vec<String>,
    #[serde(default)]
    pub wav: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongInfo {
    pub title: String,
    pub author: String,
    pub tempo: u16,
    pub time_signature: Option<String>,
}

impl SongInfo {
    /// Returns a normalized PDF filename based on author and title.
    /// Format: "<author>--@--<title>"
    /// - Capitals are replaced with lowercase
    /// - All characters which are not A-Za-z0-9 are replaced with '_'
    pub fn pdf_name_of_song(&self) -> String {
        let normalize = |s: &str| -> String {
            s.chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() {
                        c.to_ascii_lowercase()
                    } else {
                        '_'
                    }
                })
                .collect()
        };

        format!("{}--@--{}", normalize(&self.author), normalize(&self.title))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongMeta {
    pub date: Option<String>,
    pub digest: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StructureItem {
    pub id: String,
    pub item: SectionItem,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SectionItem {
    Chords(ChordsSection),
    Ref(RefSection),
    HRule(u32),
    NewColumn,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChordsSection {
    pub title: String,
    #[serde(rename = "type")]
    pub section_type: String,
    pub section_body: Option<String>,
    pub color: Option<String>,
    pub rows: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefSection {
    pub title: String,
    #[serde(rename = "type")]
    pub section_type: Option<String>,
    pub section_body: Option<String>,
    pub color: Option<String>,
    pub link: String,
}
