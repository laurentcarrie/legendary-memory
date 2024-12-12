use crate::config::model;
use crate::config::model::Section;
use crate::errors::MyError;
use std::collections::BTreeMap;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::config::model::{
    Bar, Book, BookSong, Chords, Row, Song, StructureItem, StructureItemContent,
};

use crate::config::input_model::{
    UserBook, UserChordSection, UserSong, UserStructureItem, UserStructureItemContent,
};

fn check_section_types(sections: &BTreeMap<String, Section>, song: &Song) -> Result<(), MyError> {
    // let f = |x: i32| 2 * x;

    let check = |item: &model::StructureItem| -> Result<(), MyError> {
        match &item.item {
            model::StructureItemContent::ItemRef { .. } => Ok(()),
            model::StructureItemContent::ItemChords(c) => {
                if sections.contains_key(&c.section_type) {
                    Ok(())
                } else {
                    log::info!("bad section type : '{}'", &c.section_type);
                    Err(MyError::MessageError(format!(
                        "unknown section type : {}",
                        &c.section_type
                    )))
                }
            }
            model::StructureItemContent::ItemHRule { .. } => Ok(()),
        }
        // if ! sections.contains_key()
    };
    let l = song
        .structure
        .iter()
        .fold(Ok(()), |acc, current| match acc {
            Err(e) => Err(e),
            Ok(()) => check(current),
        });
    // iter().map(|i|check(i)).collect::<Vec<_>>()? ;
    l
}

fn chord_of_string(s: String) -> String {
    s.replace("7", "sept").replace(" ", "")
}

fn bar_of_string(s: String) -> Bar {
    Bar {
        chords: s
            .split(" ")
            .map(|c| chord_of_string(c.to_string()))
            .filter(|c| c.ne(""))
            .collect(),
    }
}

fn row_of_string(barcount: u32, s: String) -> (u32, Row) {
    let bars: Vec<Bar> = s.split("|").map(|b| bar_of_string(b.to_string())).collect();
    (
        barcount + bars.len() as u32,
        Row {
            bar_number: barcount,
            bars: bars,
        },
    )
}

fn rows_of_userchordsection(barcount: u32, uc: &UserChordSection) -> (u32, Vec<Row>) {
    let bar_count_and_rows = uc.rows.iter().fold((barcount, vec![]), |mut acc, s| {
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
    let (barcount, item) = match &u.item {
        UserStructureItemContent::Chords(l) => {
            let (new_barcount, rows) = rows_of_userchordsection(barcount, &l);
            let nbcols = rows.iter().fold(0 as u32, |acc, row| {
                std::cmp::max(acc, row.bars.len() as u32)
            });
            (
                new_barcount,
                StructureItemContent::ItemChords(Chords {
                    section_title: l.section_title.clone(),
                    section_id: u.id.clone(),
                    section_type: l.section_type.clone(),
                    bar_number: barcount,
                    nb_bars: rows
                        .clone()
                        .into_iter()
                        .fold(0, |acc, row| acc + row.bars.len() as u32),
                    nbcols: nbcols,
                    nbrows: l.rows.len() as u32,
                    rows: rows,
                }),
            )
        }
        UserStructureItemContent::Ref(s) => {
            let other = {
                let others = previous
                    .iter()
                    .filter_map(|usi| match &usi.item {
                        model::StructureItemContent::ItemChords(ic) => {
                            if ic.section_id.eq(&s.link) {
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
                        panic!("found {} (instead of 1) sections with id {}", n, s.link)
                    }
                }
            };
            (
                match &other.item {
                    model::StructureItemContent::ItemChords(ic) => barcount + ic.nb_bars,
                    _ => panic!("input error"),
                },
                StructureItemContent::ItemRef(crate::config::model::Ref {
                    section_title: s.section_title.clone(),
                    section_id: u.id.clone(),
                    section_type: match &other.item {
                        model::StructureItemContent::ItemChords(ic) => ic.section_type.clone(),
                        _ => panic!("not implemented"),
                    },
                    bar_number: barcount,
                    nb_bars: match &other.item {
                        model::StructureItemContent::ItemChords(ic) => ic.nb_bars,
                        _ => panic!("not implemented"),
                    },
                }),
            )
        }
        UserStructureItemContent::HRule(u) => (
            barcount,
            model::StructureItemContent::ItemHRule(model::HRule {
                percent: match u {
                    Some(s) => *s,
                    None => 70,
                },
            }),
        ),
    };
    let si = StructureItem { item: item };
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
        pdfname: {
            crate::helpers::helpers::normalize_name(
                format!(
                    "{author}--@--{title}",
                    author = uconf.author,
                    title = uconf.title
                )
                .clone(),
            )
        },

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

pub fn decode_song(
    buildroot: &PathBuf,
    sections: &BTreeMap<String, Section>,
    filepath: &PathBuf,
) -> Result<Song, MyError> {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .clone();
    let uconf: UserSong = serde_json::from_str(&contents)?;
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
    check_section_types(&sections, &song)?;
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
                    bars: vec![
                        Bar {
                            chords: vec!["A".to_string()]
                        },
                        Bar {
                            chords: vec!["B".to_string()]
                        },
                        Bar {
                            chords: vec!["C".to_string()]
                        },
                        Bar {
                            chords: vec!["D".to_string()]
                        },
                    ]
                }
            )
        );
    }

    #[test]
    fn test2_vec() {
        let ret = rows_of_userchordsection(
            5,
            &UserChordSection {
                section_title: "".to_string(),
                section_type: "".to_string(),
                rows: vec![
                    "A|B|C|D".to_string(),
                    "Gf".to_string(),
                    "C|D|C|D".to_string(),
                ],
            },
        );
        assert_eq!(
            ret,
            (
                14,
                vec![
                    Row {
                        bar_number: 5,
                        bars: vec![
                            Bar {
                                chords: vec!["A".to_string()]
                            },
                            Bar {
                                chords: vec!["B".to_string()]
                            },
                            Bar {
                                chords: vec!["C".to_string()]
                            },
                            Bar {
                                chords: vec!["D".to_string()]
                            }
                        ]
                    },
                    Row {
                        bar_number: 9,
                        bars: vec![Bar {
                            chords: vec!["Gf".to_string()]
                        }]
                    },
                    Row {
                        bar_number: 10,
                        bars: vec![
                            Bar {
                                chords: vec!["C".to_string()]
                            },
                            Bar {
                                chords: vec!["D".to_string()]
                            },
                            Bar {
                                chords: vec!["C".to_string()]
                            },
                            Bar {
                                chords: vec!["D".to_string()]
                            }
                        ]
                    }
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        let u = UserStructureItem {
            item: UserStructureItemContent::Chords(UserChordSection {
                section_title: "".to_string(),
                section_type: "".to_string(),
                rows: vec!["Af Bfm7 | E E ".to_string(), "B".to_string()],
            }),
            id: "".to_string(),
        };
        let expected = StructureItem {
            item: StructureItemContent::ItemChords(model::Chords {
                section_title: "".to_string(),
                section_id: "".to_string(),
                section_type: "".to_string(),
                nbcols: 2,
                nbrows: 2,
                bar_number: 10,
                nb_bars: 3,
                rows: vec![
                    Row {
                        bar_number: 10,
                        bars: vec![
                            Bar {
                                chords: vec!["Af".to_string(), "Bfmsept".to_string()],
                            },
                            Bar {
                                chords: vec!["E".to_string(), "E".to_string()],
                            },
                        ],
                    },
                    Row {
                        bar_number: 12,
                        bars: vec![Bar {
                            chords: vec!["B".to_string()],
                        }],
                    },
                ],
            }),
        };
        assert_eq!((13, expected), structure_of_structure(10, &vec![], &u));
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
                    item: UserStructureItemContent::Chords(UserChordSection {
                        section_title: "".to_string(),
                        section_type: "".to_string(),
                        rows: vec!["A|B|C|D".to_string(), "E|F|G|A".to_string()],
                    }),
                    id: "blahblah".to_string(),
                },
                input_model::UserStructureItem {
                    item: UserStructureItemContent::Ref(input_model::UserRef {
                        section_title: "".to_string(),
                        link: "blahblah".to_string(),
                    }),
                    id: "".to_string(),
                },
                input_model::UserStructureItem {
                    item: UserStructureItemContent::Ref(input_model::UserRef {
                        section_title: "".to_string(),
                        link: "blahblah".to_string(),
                    }),
                    id: "".to_string(),
                },
            ],
        };
        let output = song_of_usersong(input, PathBuf::new()).unwrap();
        let expected = Song {
            title: "".to_string(),
            author: "".to_string(),
            pdfname: "--@--".to_string(),
            texfiles: vec![],
            builddir: Default::default(),
            lilypondfiles: vec![],
            wavfiles: vec![],
            date: "".to_string(),
            structure: vec![
                model::StructureItem {
                    item: model::StructureItemContent::ItemChords(Chords {
                        section_title: "".to_string(),
                        section_id: "blahblah".to_string(),
                        section_type: "".to_string(),
                        bar_number: 1,
                        nb_bars: 8,
                        nbcols: 4,
                        nbrows: 2,
                        rows: vec![
                            Row {
                                bar_number: 1,
                                bars: vec![
                                    Bar {
                                        chords: vec!["A".to_string()],
                                    },
                                    Bar {
                                        chords: vec!["B".to_string()],
                                    },
                                    Bar {
                                        chords: vec!["C".to_string()],
                                    },
                                    Bar {
                                        chords: vec!["D".to_string()],
                                    },
                                ],
                            },
                            Row {
                                bar_number: 5,
                                bars: vec![
                                    Bar {
                                        chords: vec!["E".to_string()],
                                    },
                                    Bar {
                                        chords: vec!["F".to_string()],
                                    },
                                    Bar {
                                        chords: vec!["G".to_string()],
                                    },
                                    Bar {
                                        chords: vec!["A".to_string()],
                                    },
                                ],
                            },
                        ],
                    }),
                },
                model::StructureItem {
                    item: model::StructureItemContent::ItemRef(model::Ref {
                        section_title: "".to_string(),
                        section_id: "".to_string(),
                        section_type: "".to_string(),
                        bar_number: 9,
                        nb_bars: 8,
                    }),
                },
                model::StructureItem {
                    item: model::StructureItemContent::ItemRef(model::Ref {
                        section_title: "".to_string(),
                        section_id: "".to_string(),
                        section_type: "".to_string(),
                        bar_number: 17,
                        nb_bars: 8,
                    }),
                },
            ],
        };
        assert_eq!(expected.structure.get(0), output.structure.get(0));
        assert_eq!(expected.structure.get(1), output.structure.get(1));
        assert_eq!(expected.structure.get(2), output.structure.get(2));
        assert_eq!(expected.structure.get(3), output.structure.get(3));
        assert_eq!(expected, output);
    }
}
