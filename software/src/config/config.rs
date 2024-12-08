use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::config::model::{Book, BookSong, Song, StructureItem, StructureItemContent};

use crate::config::input_model::{UserBook, UserSong, UserStructureItem, UserStructureItemContent};

fn structure_of_structure(u: &UserStructureItem) -> StructureItem {
    return StructureItem {
        texname: u.texname.clone(),
        sectiontype: u.sectiontype.clone(),
        text: u.text.clone(),
        content: match &u.content {
            UserStructureItemContent::Chords(l) => {
                let l: Vec<_> = l
                    .iter()
                    .map(|c| c.as_str().split("|"))
                    .flatten()
                    .map(|c| c.replace("7", "sept").replace(" ", ""))
                    .map(|c| c.to_string())
                    .filter(|c| c.len() > 0)
                    .collect();
                StructureItemContent::Chords(l.clone().to_owned())
            }
        },
    };
}

/// read a json file and returns a Book
pub fn decode_book(buildroot: &PathBuf, filepath: &PathBuf) -> Result<Book, Error> {
    log::debug!("read book {:?}", &filepath);
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .clone();
    let uconf: UserBook = serde_json::from_str(&contents)
        .expect(format!("read json {}", filepath.display()).as_str());
    // let j = serde_json::to_string(&uconf)?;
    // {
    //     let mut p2 = filepath.clone();
    //     p2.set_file_name("book.json");
    //     fs::write(p2, j);
    // }

    // dbg!(&uconf);
    let mut book_builddir = buildroot.clone();
    book_builddir.push("books");
    book_builddir.push(&uconf.title);
    let book = Book {
        title: uconf.title,
        songs: uconf
            .songs
            .iter()
            .map(|ub| BookSong {
                title: ub.title.clone(),
                author: ub.author.clone(),
            })
            .collect(),
        builddir: book_builddir,
    };
    Ok(book)
}

pub fn decode_song(buildroot: &PathBuf, filepath: &PathBuf) -> Result<Song, Error> {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .clone();
    let uconf: UserSong = serde_json::from_str(&contents)
        .expect(format!("read json {}", filepath.display()).as_str());
    // let j = serde_json::to_string(&uconf)?;
    // {
    //     let mut p2 = filepath.clone();
    //     p2.set_file_name("song.json");
    //     fs::write(p2, j);
    // }

    // dbg!(&uconf);
    let mut song_builddir = buildroot.clone();
    song_builddir.push("songs");
    let parent_title = filepath
        .parent()
        .expect("has title parent")
        .file_name()
        .expect("has file name")
        .to_str()
        .expect("has name");
    let parent_author = filepath
        .parent()
        .expect("has title parent")
        .parent()
        .expect("has author parent")
        .file_name()
        .expect("has file name")
        .to_str()
        .expect("has name");
    song_builddir.push(&parent_author);
    song_builddir.push(&parent_title);
    let song = Song {
        texfiles: uconf.texfiles,
        author: uconf.author,
        title: uconf.title,
        builddir: song_builddir,
        lilypondfiles: uconf.lilypondfiles,
        wavfiles: uconf.wavfiles,
        date: uconf.date,
        structure: {
            match uconf.structure {
                Some(s) => s.iter().map(structure_of_structure).collect(),
                None => vec![],
            }
        },
    };
    Ok(song)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1_bar_of_string() {
        let s = "A".to_string();
        assert_eq!(
            bar_of_string(&s),
            Bar {
                chords: vec!["A".to_string()]
            }
        );
    }
    #[test]
    fn test2_bar_of_string() {
        let s = "A B".to_string();
        assert_eq!(
            bar_of_string(&s),
            Bar {
                chords: vec!["A".to_string(), "B".to_string()]
            }
        );
    }

    #[test]
    fn test_row_of_string() {
        let v = vec![
            "A B".to_string(),
            "C D D2".to_string(),
            "E".to_string(),
            "F G H".to_string(),
        ];
        let e = Row {
            bars: vec![
                Bar {
                    chords: vec!["A".to_string(), "B".to_string()],
                },
                Bar {
                    chords: vec!["C".to_string(), "D".to_string(), "D2".to_string()],
                },
                Bar {
                    chords: vec!["E".to_string()],
                },
                Bar {
                    chords: vec!["F".to_string(), "G".to_string(), "H".to_string()],
                },
            ],
        };
        assert_eq!(row_of_vstring(&v), e);
    }
}
