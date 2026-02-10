/// LilyPond source file node (`.ly`).
pub mod lilypond;
/// LilyPond-to-TeX intermediate file node (`.lytex`).
pub mod lytex;
/// PDF output node, built by running `lualatex`.
pub mod pdf;
/// Node that copies a built PDF to the delivery directory.
pub mod pdfcopyfile;
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

pub use lilypond::LilypondFile;
pub use lytex::LyTexFile;
pub use pdf::PdfFile;
pub use pdfcopyfile::PdfCopyFile;
pub use songtikz::SongTikz;
pub use songyml::SongYml;
pub use strudel::StrudelFile;
pub use tex::TexFile;
pub use texoflilypond::TexOfLilypond;
