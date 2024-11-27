use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::{Book, World};
use crate::helpers::helpers::{pdfname_of_book, pdfname_of_song};

pub fn generate_main_book(book: &Book) -> Result<(), Error> {
    log::debug!("generate main.tex in {}", book.builddir.display());
    let mut p: PathBuf = book.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    let _ = pdfname_of_book(&book);
    p.push("main.tex");
    let mut output = File::create(p)?;
    // dbg!(&output);
    // {
    //     Ok(x) => (x),
    //     Err(e) => return Err(e),
    // };
    // // write!(output, "Rust\n💖\nFun");

    write!(
        output,
        r###"
%\documentclass[a4paper, 12 pt, showframe, french]{{scrartcl}}
\documentclass[a4paper, 10 pt]{{book}}
\usepackage[left=1cm,right=1cm,top=1cm,bottom=2cm]{{geometry}}
\usepackage{{import}}
\usepackage{{fancyhdr}}
\usepackage{{lastpage}}
\pagestyle{{fancy}}
\fancyhf{{}}
\newcommand{{\makesongtitle}}{{}}
\newcommand{{\songlastupdate}}{{}}
\newcommand{{\songtoday}}{{}}
\import{{../}}{{preamble}}
\import{{../}}{{chords}}

\begin{{document}}

\tableofcontents
\newpage

"###
    )?;

    for song in &book.songs {
        write!(
            output,
            r###"
\section{{ {author_sanitized} / {title_sanitized} }}
\lfoot{{ {author_sanitized} / {title_sanitized} }}
\import{{../../songs/{author}/{title}/}}{{data.tex}}
\makesongtitle
\import{{../../songs/{author}/{title}/}}{{body.tex}}
\newpage

    "###,
            author = song.author,
            author_sanitized = song.author.replace("_", "\\_"),
            title = song.title,
            title_sanitized = song.title.replace("_", "\\_"),
        )?;
    }

    write!(
        output,
        r###"
hello world
\end{{document}}
"###
    )?;
    Ok(())
}

pub fn generate_book_omakefile(book: &Book) -> Result<(), Error> {
    log::debug!("generate Omakefile in {}", book.builddir.display());
    let mut p: PathBuf = book.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    let pdfname = pdfname_of_book(&book);
    p.push("OMakefile");
    let mut output = File::create(p)?;
    // dbg!(&output);
    // {
    //     Ok(x) => (x),
    //     Err(e) => return Err(e),
    // };
    // // write!(output, "Rust\n💖\nFun");

    write!(
        output,
        r###".PHONY: pdf wav midi clean
clean:
    bash  $(buildroot)/make_clean.sh

"###
    )?;

    write!(
        output,
        r#"

pdf : {pdfname}.pdf

main.pdf : main.tex
	bash $(buildroot)/make_pdf.sh main

{pdfname}.pdf : main.pdf
    cp main.pdf {pdfname}.pdf
"#,
        pdfname = pdfname
    )?;

    Ok(())
}
