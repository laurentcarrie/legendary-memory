use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::Book;
use crate::generate::handlebars_helpers::get_handlebar;
// use crate::helpers::helpers::pdfname_of_book;

pub fn generate_main_book(book: &Book) -> Result<(), Error> {
    log::debug!("generate main.tex in {}", book.builddir.display());
    let mut p: PathBuf = book.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("main.tex");
    let mut output = File::create(p)?;
    let template =
        String::from_utf8(include_bytes!("../../others/texfiles/mainbook.tex").to_vec()).unwrap();

    let mut h = get_handlebar()?;
    h.register_template_string("t1", template).unwrap();
    let output_data = h.render("t1", book).unwrap();
    let _ = output.write(output_data.as_bytes()).unwrap();

    Ok(())
}

pub fn generate_book_omakefile(book: &Book) -> Result<(), Error> {
    log::debug!("generate Omakefile in {}", book.builddir.display());
    let mut p: PathBuf = book.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("OMakefile");
    let mut output = File::create(p)?;
    let template =
        String::from_utf8(include_bytes!("../../others/makefiles/omakefile_book").to_vec())
            .unwrap();

    let mut h = get_handlebar()?;
    h.register_template_string("t1", template).unwrap();
    let output_data = h.render("t1", book).unwrap();
    let _ = output.write(output_data.as_bytes()).unwrap();

    Ok(())
}
