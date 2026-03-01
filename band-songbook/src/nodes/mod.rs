/// Node that overlays click sounds onto a song MP3 for verification.
pub mod clickcheckmp3;
/// Click track analysis node that detects ticks in an MP3 and outputs tick offsets as YAML.
pub mod clickyml;
/// Node that copies a built PDF to the delivery directory.
pub mod copyfile;
/// LilyPond source file node (`.ly`).
pub mod lilypond;
/// LilyPond-to-TeX intermediate file node (`.lytex`).
pub mod lytex;
/// MP3 audio file node (`.mp3`).
pub mod mp3;
/// PDF output node, built by running `lualatex`.
pub mod pdf;
/// TikZ chord chart node (`song.tikz`).
pub mod songtikz;
/// Song root node that expands `song.yml` into the build graph.
pub mod songyml;
/// Strudel HTML node with interactive drum patterns.
pub mod strudel;
/// LaTeX source file node (`.tex`).
pub mod tex;
/// TeX output from LilyPond compilation.
pub mod texoflilypond;

pub use clickcheckmp3::ClickCheckMp3;
pub use clickyml::ClickYml;
pub use copyfile::CopyFile;
pub use lilypond::LilypondFile;
pub use lytex::LyTexFile;
pub use mp3::Mp3;
pub use pdf::PdfFile;
pub use songtikz::SongTikz;
pub use songyml::SongYml;
pub use strudel::StrudelFile;
pub use tex::TexFile;
pub use texoflilypond::TexOfLilypond;
