use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A complete song definition, deserialized from `song.yml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Song {
    /// Source files (LilyPond, TeX, WAV) associated with this song.
    pub files: SongFiles,
    /// Title, author, tempo, and time signature.
    pub info: SongInfo,
    /// Optional metadata such as date and content digest.
    pub meta: SongMeta,
    /// Ordered list of sections that make up the song structure.
    pub structure: Vec<StructureItem>,
}

/// Source file references for a song.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SongFiles {
    /// LilyPond notation files (`.ly`).
    #[serde(default)]
    pub lilypond: Vec<String>,
    /// LaTeX source files (`.tex`).
    #[serde(default)]
    pub tex: Vec<String>,
    /// Audio files (`.wav`).
    #[serde(default)]
    pub wav: Vec<String>,
    /// Whether this song has a click track (`clicks.mp3`).
    #[serde(default)]
    pub has_clicks: bool,
    /// Whether this song has a song recording (`song.mp3`).
    #[serde(default)]
    pub has_mp3: bool,
}

/// Core song metadata: title, author, tempo, and time signature.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SongInfo {
    /// Song title.
    pub title: String,
    /// Song author or band name.
    pub author: String,
    /// Tempo in beats per minute.
    pub tempo: u16,
    /// Time signature (e.g. `"4/4"`), if specified.
    pub time_signature: Option<String>,
    /// Free-form tags for categorizing the song.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl SongInfo {
    /// Returns a normalized file stem based on author and title.
    /// Format: `{author}--@--{title}`
    /// - Capitals are replaced with lowercase
    /// - All characters which are not A-Za-z0-9 are replaced with '_'
    pub fn file_stem_of_song(&self) -> String {
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

/// Optional metadata for a song (date, content digest).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SongMeta {
    /// Date the song was added or last modified.
    pub date: Option<String>,
    /// MD5 digest of the song content for change detection.
    pub digest: Option<String>,
}

/// A named section in the song structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StructureItem {
    /// Unique identifier for this section (e.g. `"intro"`, `"couplet1"`).
    pub id: String,
    /// The section content.
    pub item: SectionItem,
}

/// The different kinds of sections that can appear in a song structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SectionItem {
    /// A section with chord rows.
    Chords(ChordsSection),
    /// A reference to another section (shares its chord content).
    Ref(RefSection),
    /// A horizontal rule with a given width percentage.
    HRule(u32),
    /// A column break in multi-column layouts.
    NewColumn,
}

/// A section containing chord rows and optional drum patterns.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChordsSection {
    /// Display title for this section.
    pub title: String,
    /// Section type (e.g. `"intro"`, `"couplet"`, `"refrain"`).
    #[serde(rename = "type")]
    pub section_type: String,
    /// Optional body text displayed below the section header.
    pub section_body: Option<String>,
    /// Background color name (X11 color).
    pub color: Option<String>,
    /// Chord rows, each in `"A|Bm|C#|x2"` format.
    pub rows: Vec<String>,
    /// Optional drum pattern sequence for Strudel generation.
    pub drum_sequence: Option<DrumSequence>,
}

/// A single item in a drum sequence: a pattern name repeated N times.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DrumSequenceItem {
    /// Number of consecutive bars using this pattern.
    pub repeat: u32,
    /// Name of the drum pattern (must exist in a library directory).
    pub pattern: String,
}

/// A sequence of drum pattern items for a section.
pub type DrumSequence = Vec<DrumSequenceItem>;

/// A section that references another section's chord content.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefSection {
    /// Display title for this reference section.
    pub title: String,
    /// Optional override of the section type.
    #[serde(rename = "type")]
    pub section_type: Option<String>,
    /// Optional body text.
    pub section_body: Option<String>,
    /// Background color name (X11 color).
    pub color: Option<String>,
    /// ID of the [`ChordsSection`] this references.
    pub link: String,
}

/// An item in the world: either a successfully parsed song or an error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldItem {
    /// A successfully parsed song.
    Song(Song),
    /// An error message from reading or parsing a song file.
    Error(String),
}

/// Tick offsets parsed from `clicks.yml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClicksDefinition {
    /// Absolute time offsets (in seconds) of each detected tick from the start of the audio.
    pub ticks: Vec<f64>,
}

/// The world: a collection of song paths and their parse results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    /// List of (relative path, world item) pairs.
    pub items: Vec<(PathBuf, WorldItem)>,
}
