/// Handlebars template helper functions for LaTeX generation.
pub mod handlebar_helpers;
/// Conversion from song structure to Strudel bar sequences.
pub mod sequence;

pub use handlebar_helpers::register_helpers;
pub use sequence::strudel_sequence_of_song;
