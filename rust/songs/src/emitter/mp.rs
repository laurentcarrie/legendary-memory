// use crate::generated::mp_code::{make_flat, make_sharp};
// use std::fs::File;
// use std::io::{Error, Write};
//
// pub fn add_mp_macros(mut output: File) -> Result<(), Error> {
//     let data = r###"
//     prologues:=3;
//     outputtemplate := "mps/main-%c.mps";
//     outputformat := "mps";
//     input boxes ;
//     input TEX ;
//     %verbatimtex \nofiles etex;
//     verbatimtex
//     \documentclass{article}
//         %%\usepackage{lmodern}
//     \usepackage[tt=false]{libertine}
//     \usepackage[libertine]{newtxmath}
//     \usepackage{amsmath}
//     \begin{document}
//     etex
//
//         %fontmapfile "=lm-ec.map";
//
//
//     numeric  chord_glyph_scale ;
//     chord_glyph_scale:=2. ;
// "###;
//     write!(output, "{}\n", data);
//     write!(output, "{}\n", make_flat())?;
//     write!(output, "{}\n", make_sharp())?;
//
//     write!(output, "\nend.\n")?;
//     Ok(())
// }
