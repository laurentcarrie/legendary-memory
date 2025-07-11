use crate::model::model::{Book, BookSong, Song, World};
use human_sort::compare;
use std::cmp::Ordering;

pub fn pdfname_of_book(book: &Book) -> String {
    let pdfname = &book.title;
    normalize_name(pdfname.to_owned())
}

pub fn normalize_pdf_name(author: &String, title: &String) -> String {
    normalize_name(format!("{author}--@--{title}").clone())
}

pub fn normalize_name(input: String) -> String {
    let mut output = input.clone();
    output.make_ascii_lowercase();
    output = output
        .replace(" ", "_")
        .replace("/", "_")
        .replace(".", "_")
        .replace(")", "_")
        .replace("(", "_")
        .replace("'", "_");
    output
}

pub fn song_of_booksong(world: &World, bs: &BookSong) -> Result<Song, Box<dyn std::error::Error>> {
    for song in &world.songs {
        if compare(song.author.as_str(), bs.author.as_str()) == Ordering::Equal
            && compare(song.title.as_str(), bs.title.as_str()) == Ordering::Equal
        {
            return Ok(song.clone());
        }
    }
    Err(format!("could not find {bs:?}").into())
}
