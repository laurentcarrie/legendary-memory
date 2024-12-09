use crate::config::{input_model, model};
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

fn structure_of_structure(
    barcount: u32,
    previous: &Vec<StructureItem>,
    u: &UserStructureItem,
) -> (u32, StructureItem) {
    let (barcount, content) = match &u.content {
        UserStructureItemContent::Chords(l) => {
            let (new_barcount, rows) = rows_of_vec_string(barcount, &l);
            let nbcols = rows.iter().fold(1000 as u32, |acc, row| {
                std::cmp::min(acc, row.chords.len() as u32)
            });
            (
                new_barcount,
                StructureItemContent::ItemChords(Chords {
                    bar_number: barcount,
                    nb_bars: rows
                        .clone()
                        .into_iter()
                        .fold(0, |acc, row| acc + row.chords.len() as u32),
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
        UserStructureItemContent::Ref(s) => {
            let other = {
                let others = previous
                    .iter()
                    .filter_map(|usi| match &usi.content {
                        model::StructureItemContent::ItemChords(ic) => {
                            if usi.section_id.eq(s) {
                                Some(usi)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .clone();
                match others.len() {
                    1 => others.get(0).unwrap().clone(),
                    n => {
                        panic!(
                            "found {} (instead of 1) sections with id {}",
                            others.len(),
                            s
                        )
                    }
                }
                // .get(0)
                // .expect(format!("ref {}", s).as_str())
                // .clone();
            };
            (
                0,
                StructureItemContent::ItemRef(crate::config::model::Ref {
                    bar_number: barcount,
                    nb_bars: match &other.content {
                        model::StructureItemContent::ItemChords(ic) => ic.nb_bars,
                        _ => 99999,
                    },
                }),
            )
        }
    };
    let si = StructureItem {
        title: u.title.clone(),
        section_id: u.section_id.clone(),
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

fn song_of_usersong(uconf: UserSong, song_builddir: PathBuf) -> Result<Song, Error> {
    let song = Song {
        texfiles: uconf.texfiles.clone(),
        author: uconf.author.clone(),
        title: uconf.title.clone(),
        builddir: song_builddir,
        lilypondfiles: uconf.lilypondfiles.clone(),
        wavfiles: uconf.wavfiles.clone(),
        date: uconf.date.clone(),
        structure: {
            match uconf.structure {
                ref s => {
                    let acc = s.iter().fold((1, Vec::new()), |mut acc, s| {
                        let (newcount, si) = structure_of_structure(acc.0, &(acc.1), s);
                        acc.0 = newcount;
                        acc.1.push(si);
                        acc
                    });
                    acc.1
                }
            }
        },
    };
    Ok(song)
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
    let song = song_of_usersong(uconf, song_builddir)?;
    Ok(song)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{input_model, model};

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
            section_id: "".to_string(),
            sectiontype: "".to_string(),
            content: UserStructureItemContent::Chords(vec!["A".to_string(), "B".to_string()]),
            text: "".to_string(),
        };
        let expected = StructureItem {
            title: "".to_string(),
            section_id: "".to_string(),
            text: "".to_string(),
            sectiontype: "".to_string(),
            content: StructureItemContent::ItemChords(model::Chords {
                nbcols: 1,
                nbrows: 2,
                bar_number: 10,
                nb_bars: 2,
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
        assert_eq!((12, expected), structure_of_structure(10, &vec![], &u));
    }

    #[test]
    fn test_4() {
        let input = UserSong {
            title: "".to_string(),
            author: "".to_string(),
            texfiles: vec![],
            lilypondfiles: vec![],
            wavfiles: vec![],
            date: "".to_string(),
            structure: vec![
                input_model::UserStructureItem {
                    title: "".to_string(),
                    content: UserStructureItemContent::Chords(vec![
                        "A|B|C|D".to_string(),
                        "E|F|G|A".to_string(),
                    ]),
                    section_id: "blahblah".to_string(),
                    sectiontype: "".to_string(),
                    text: "".to_string(),
                },
                input_model::UserStructureItem {
                    title: "".to_string(),
                    section_id: "".to_string(),
                    sectiontype: "".to_string(),
                    content: UserStructureItemContent::Ref("blahblah".to_string()),
                    text: "".to_string(),
                },
            ],
        };
        let output = song_of_usersong(input, PathBuf::new()).unwrap();
        let expected = Song {
            title: "".to_string(),
            author: "".to_string(),
            texfiles: vec![],
            builddir: Default::default(),
            lilypondfiles: vec![],
            wavfiles: vec![],
            date: "".to_string(),
            structure: vec![
                model::StructureItem {
                    title: "".to_string(),
                    section_id: "blahblah".to_string(),
                    text: "".to_string(),
                    sectiontype: "".to_string(),
                    content: model::StructureItemContent::ItemChords(Chords {
                        bar_number: 1,
                        nb_bars: 8,
                        colspec: "C|C|C|C".to_string(),
                        nbcols: 4,
                        nbrows: 2,
                        CodeBefore: "\\rowcolor{\\lolocolor!100}{1-2}\n\\cellcolor{white}{1-1,2-1}"
                            .to_string(),
                        rows: vec![
                            Row {
                                bar_number: 1,
                                chords: vec![
                                    "A".to_string(),
                                    "B".to_string(),
                                    "C".to_string(),
                                    "D".to_string(),
                                ],
                            },
                            Row {
                                bar_number: 5,
                                chords: vec![
                                    "E".to_string(),
                                    "F".to_string(),
                                    "G".to_string(),
                                    "A".to_string(),
                                ],
                            },
                        ],
                    }),
                },
                model::StructureItem {
                    title: "".to_string(),
                    section_id: "".to_string(),
                    text: "".to_string(),
                    sectiontype: "".to_string(),
                    content: model::StructureItemContent::ItemRef(model::Ref {
                        bar_number: 9,
                        nb_bars: 8,
                    }),
                },
            ],
        };
        assert_eq!(expected.structure.get(0), output.structure.get(0));
        assert_eq!(expected, output);
    }
}
