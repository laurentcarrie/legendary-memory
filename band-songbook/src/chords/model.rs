/// Indicates an accidental (flat, sharp, or none)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accidental {
    Flat,
    Sharp,
    None,
}

/// Indicates a chord alteration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alteration {
    None,
    Nofith,
    Six,
    Seven,
    MajorSeven,
    Sus2,
    Sus4,
    Dim,
}

/// Represents a musical chord
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub name: String,
    pub accidental: Accidental,
    pub minor: bool,
    pub alteration: Alteration,
}

/// Represents a rest (silence)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rest {
    pub duration: u32,
}

/// Represents a repeat marker (e.g., x2, x3)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repeat {
    pub n: u32,
}

/// An item in a bar: either a chord or a rest
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarItem {
    Chord(Chord),
    Rest(Rest),
}

/// Represents a bar (measure) containing chords with their durations
#[derive(Debug, Clone)]
pub struct Bar {
    pub items: Vec<BarItem>,
}

/// Represents a parsed row with bars and an optional repeat
#[derive(Debug, Clone)]
pub struct ParsedRow {
    pub bars: Vec<Bar>,
    pub repeat: Repeat,
}
