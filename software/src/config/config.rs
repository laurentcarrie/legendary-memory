use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::config::model::{
    Book, BookSong, Chords, Row, Song, StructureItem, StructureItemContent,
};

use crate::config::input_model::{UserBook, UserSong, UserStructureItem, UserStructureItemContent};

fn row_of_string(barcount: u32, s: String) -> (u32, Row) {
    let l: Vec<_> = s
        .split("|")
        .map(|c| c.replace("7", "sept").replace(" ", ""))
        .map(|c| c.to_string())
        .filter(|c| c.len() > 0)
        .collect();
    // while l.len() < 4 {
    //     l.push("".to_string());
    // }
    (
        barcount + l.len() as u32,
        Row {
            bar_number: barcount,
            chords: l,
        },
    )
}

fn rows_of_vec_string(barcount: u32, rows: &Vec<String>) -> (u32, Vec<Row>) {
    let bar_count_and_rows = rows.iter().fold((barcount, vec![]), |mut acc, s| {
        let ret = row_of_string(acc.0, s.to_string());
        acc.0 = ret.0;
        acc.1.push(ret.1);
        acc
    });
    bar_count_and_rows
}

fn structure_of_structure(barcount: u32, u: &UserStructureItem) -> (u32, StructureItem) {
    let (barcount, content) = match &u.content {
        UserStructureItemContent::Chords(l) => {
            let (barcount, rows) = rows_of_vec_string(barcount, &l);
            let nbcols = rows.iter().fold(1000 as u32, |acc, row| {
                std::cmp::min(acc, row.chords.len() as u32)
            });
            (
                barcount,
                StructureItemContent::ItemChords(Chords {
                    colspec: (0..nbcols)
                        .map(|_| "C".to_string())
                        .collect::<Vec<_>>()
                        .join("|"),
                    nbcols: nbcols,
                    nbrows: l.len() as u32,
                    CodeBefore: format!(
                        "\
                    \\rowcolor{{\\lolocolor{sectiontype}!100}}{{1-{nbrows}}}\n\
                    \\cellcolor{{white}}{{{cellcolor}}}",
                        sectiontype = u.sectiontype,
                        nbrows = l.len(),
                        cellcolor = (0..l.len())
                            .map(|i| format!("{}-1", i + 1))
                            .collect::<Vec<_>>()
                            .join(",")
                    ),

                    rows: rows,
                }),
            )
        }
    };
    let si = StructureItem {
        title: u.title.clone(),
        texname: u.texname.clone(),
        sectiontype: u.sectiontype.clone(),
        text: u.text.clone(),
        content: content,
    };
    (barcount, si)
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
                Some(s) => {
                    let acc = s.iter().fold((1, Vec::new()), |mut acc, s| {
                        let (newcount, si) = structure_of_structure(acc.0, s);
                        acc.0 = newcount;
                        acc.1.push(si);
                        acc
                    });
                    acc.1
                }
                None => vec![],
            }
        },
    };
    Ok(song)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model;

    #[test]
    fn test1_bar_of_string() {
        let ret = row_of_string(1, "A|B|C|D".to_string());
        assert_eq!(
            ret,
            (
                5,
                Row {
                    bar_number: 1,
                    chords: vec![
                        "A".to_string(),
                        "B".to_string(),
                        "C".to_string(),
                        "D".to_string()
                    ]
                }
            )
        );
    }

    #[test]
    fn test2_vec() {
        let ret = rows_of_vec_string(
            5,
            &vec![
                "A|B|C|D".to_string(),
                "Gf".to_string(),
                "C|D|C|D".to_string(),
            ]
            .clone(),
        );
        assert_eq!(
            ret,
            (
                14,
                vec![
                    Row {
                        bar_number: 5,
                        chords: vec![
                            "A".to_string(),
                            "B".to_string(),
                            "C".to_string(),
                            "D".to_string()
                        ]
                    },
                    Row {
                        bar_number: 9,
                        chords: vec!["Gf".to_string(),]
                    },
                    Row {
                        bar_number: 10,
                        chords: vec![
                            "C".to_string(),
                            "D".to_string(),
                            "C".to_string(),
                            "D".to_string()
                        ]
                    }
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        let u = UserStructureItem {
            title: "".to_string(),
            texname: "".to_string(),
            sectiontype: "".to_string(),
            content: UserStructureItemContent::Chords(vec!["A".to_string(), "B".to_string()]),
            text: "".to_string(),
        };
        let expected = StructureItem {
            title: "".to_string(),
            texname: "".to_string(),
            text: "".to_string(),
            sectiontype: "".to_string(),
            content: StructureItemContent::ItemChords(model::Chords {
                nbcols: 1,
                nbrows: 2,
                colspec: "C".to_string(),
                CodeBefore: "\\rowcolor{\\lolocolor!100}{1-2}\n\\cellcolor{white}{1-1,2-1}"
                    .to_string(),
                rows: vec![
                    Row {
                        bar_number: 10,
                        chords: vec!["A".to_string()],
                    },
                    Row {
                        bar_number: 11,
                        chords: vec!["B".to_string()],
                    },
                ],
            }),
        };
        //fn structure_of_structure(barcount: u32, u: &UserStructureItem) -> (u32, StructureItem) {
        assert_eq!((12, expected), structure_of_structure(10, &u));
    }
}
