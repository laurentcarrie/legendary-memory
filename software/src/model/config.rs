use crate::errors::MyError;
use crate::helpers::helpers::normalize_pdf_name;
use crate::model::model;
use crate::model::model::{Section, TimeSignature};
// use handlebars::template::Parameter::Path;
use human_sort::compare;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::model::model::{
    Bar, Book, BookSong, Chords, Row, Song, StructureItem, StructureItemContent,
};

use crate::model::input_model::{
    UserBook, UserBookWithPath, UserChordSection, UserSong, UserSongWithPath, UserStructureItem,
    UserStructureItemContent,
};

fn time_signature_of_string(input: Option<String>) -> Result<TimeSignature, MyError> {
    match input {
        None => Ok(TimeSignature { top: 4, low: 4 }),
        Some(s) => {
            let re = Regex::new("(.*)/(.*)").unwrap();
            let ss = re.captures(&s);
            match ss {
                None => Err(MyError::MessageError(format!(
                    "'{} is not a valid time signature",
                    s
                ))),
                Some(caps) => {
                    let top = (&caps[0]).to_string().parse::<u8>()?;
                    let low = (&caps[1]).to_string().parse::<u8>()?;
                    Ok(TimeSignature { top: top, low: low })
                }
            }
        }
    }
}

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
    s.replace("7", "sept")
        .replace(" ", "")
        .replace("2", "deux")
        .replace("3", "trois")
        .replace("4", "quatre")
}

fn bar_of_string(s: String) -> Result<Bar, MyError> {
    // time signature
    let re_ts = Regex::new("time_signature\\((.*)/(.*)\\)(.*)").unwrap();
    let (s, time_signature) = match re_ts.captures(&s) {
        Some(caps) => {
            (
                (&caps[3]).to_string(),
                Some(TimeSignature {
                    top: (&caps[1]).to_string().parse::<u8>()?,
                    low: (&caps[2]).to_string().parse::<u8>()?,
                }),
            )
        }
        None => (s, None),
    };

    let re = Regex::new("\\{latex (.*)\\}").unwrap();
    let ss = re.captures(&s);
    match ss {
        Some(caps) => Ok(Bar {
            chords: vec![(&caps[1]).to_string()],
            time_signature: time_signature,
        }),
        None => Ok(Bar {
            chords: s
                .split(" ")
                .map(|c| chord_of_string(c.to_string()))
                .filter(|c| c.ne(""))
                .collect(),
            time_signature: time_signature,
        }),
    }
}

/// takes a string, eg " |A|B|C|D|x2"
/// and returns the number of bars, and a row
/// the number of bars takes the repeat into account, in this example 8
fn row_of_string(barcount: u32, s: String) -> (u32, Row) {
    let bars: Result<Vec<Bar>, MyError> = s
        .split("|")
        .map(|b| bar_of_string(b.to_string()))
        .into_iter()
        .collect();
    // the string looks like "| A | B |C | x2"
    // the repeat if any is in the last cell
    let mut bars = bars.unwrap();
    let repeat = match bars.pop() {
        None => 1,
        Some(mut b) => {
            // todo : we assume that we have | x3 |, what happens if | blahblah x3 |
            match b.chords.pop() {
                None => 1,
                Some(s) => match s.as_str() {
                    "xdeux" => 2,
                    "xtrois" => 3,
                    "xquatre" => 4,
                    _ => {
                        b.chords.push(s);
                        bars.push(b);
                        1
                    }
                },
            }
        }
    };

    (
        barcount + repeat * bars.len() as u32,
        Row {
            repeat: repeat,
            row_start_bar_number: barcount,
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
                    section_body: {
                        match &l.section_body {
                            None => "".to_string(),
                            Some(s) => s.clone(),
                        }
                    },
                    row_start_bar_number: barcount,
                    nb_bars: rows
                        .clone()
                        .into_iter()
                        .fold(0, |acc, row| acc + row.bars.len() as u32 * row.repeat),
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
                    1 => *others.get(0).unwrap(),
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
                StructureItemContent::ItemRef(crate::model::model::Ref {
                    section_title: s.section_title.clone(),
                    section_id: u.id.clone(),
                    section_body: {
                        match &s.section_body {
                            None => "".to_string(),
                            Some(s) => s.clone(),
                        }
                    },
                    section_type: match &other.item {
                        model::StructureItemContent::ItemChords(ic) => match &s.section_type {
                            Some(s) => s.clone(),
                            None => ic.section_type.clone(),
                        }, // ic.section_type.clone(),
                        _ => panic!("not implemented"),
                    },
                    row_start_bar_number: barcount,
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

fn build_relative_path_of_source_absolute_path(
    songdir: &PathBuf,
    builddir: &PathBuf,
    p: PathBuf,
) -> Result<PathBuf, MyError> {
    let songdir = songdir
        .to_str()
        .ok_or(MyError::MessageError("? songdir".to_string()))?;
    let p = p
        .parent()
        .ok_or(MyError::MessageError("? no parent".to_string()))?
        .to_str()
        .ok_or(MyError::MessageError("? songdir".to_string()))?
        .to_string();
    let p = p.replace(format!("{}/", songdir).as_str(), "songs/");
    let mut ret = builddir.clone();
    ret.push(p);
    Ok(ret)
}

fn song_exists(
    author: &String,
    title: &String,
    songs: &Vec<UserSongWithPath>,
) -> Option<UserSongWithPath> {
    for song in songs {
        if compare(author.as_str(), song.song.author.as_str()) == Ordering::Equal {
            if compare(title.as_str(), song.song.title.as_str()) == Ordering::Equal {
                return Some(song.clone());
                // let p = build_relative_path_of_source_absolute_path(
                //     songdir,
                //     builddir,
                //     PathBuf::from(song.path.clone()),
                // );
                // match p {
                //     Ok(p) => {
                //         let x = p.as_path().to_str().map(|x| x.to_string());
                //         return x;
                //     }
                //     _ => (),
                // }
            }
        }
    }
    // log::info!("DID NOT FIND '{}' '{}'", author, title);
    // for song in songs {
    //     log::info!(
    //         "{}:{} '{}' '{}'",
    //         file!(),
    //         line!(),
    //         song.author.as_str(),
    //         song.title.as_str()
    //     );
    // }
    None
}
/// read a json file and returns a Book
pub fn decode_book(
    songdir: &PathBuf,
    buildroot: &PathBuf,
    filepath: &PathBuf,
    songs: &Vec<UserSongWithPath>,
) -> Result<(Book, UserBookWithPath), MyError> {
    log::debug!("read book {:?}", &filepath);
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .clone();
    let uconf: UserBook = serde_json::from_str(&contents)
        .expect(format!("read json {}", filepath.display()).as_str());

    let mut book_builddir = buildroot.clone();
    book_builddir.push("books");
    book_builddir.push(&uconf.title);
    let (songs, errors): (
        Vec<Result<BookSong, MyError>>,
        Vec<Result<BookSong, MyError>>,
    ) = uconf
        .songs
        .clone()
        .into_iter()
        .map(|ub| match song_exists(&ub.author, &ub.title, songs) {
            Some(uswp) => {
                let path = build_relative_path_of_source_absolute_path(
                    &songdir,
                    &buildroot,
                    PathBuf::from(uswp.path),
                )?
                .to_str()
                .ok_or(MyError::MessageError("huh?".to_string()))?
                .to_string();
                Ok(BookSong {
                    title: ub.title.clone(),
                    author: ub.author.clone(),
                    pdfname: normalize_pdf_name(&ub.author, &ub.title),
                    path: path,
                })
            }
            None => Err(MyError::MessageError(format!(
                "invalid or missing song : {} / {}",
                ub.author.clone(),
                ub.title.clone()
            ))),
        })
        .partition(Result::is_ok);
    let songs = songs.iter().map(|s| s.as_ref().unwrap().clone()).collect();
    let mut errors: Vec<_> = errors.into_iter().map(|s| s.err().unwrap()).collect();

    match errors.pop() {
        None => {
            let book = Book {
                title: uconf.title.clone(),
                songs: songs,
                builddir: book_builddir,
                pdfname: format!("{}.pdf", uconf.title.clone()).to_string(),
            };
            let ubook_with_path = UserBookWithPath {
                book: uconf.clone(),
                path: filepath.to_str().unwrap().to_string(),
            };
            Ok((book, ubook_with_path))
        }
        Some(e) => Err(e),
    }
}

fn song_of_usersong(
    uconf: UserSong,
    song_srcdir: PathBuf,
    song_builddir: PathBuf,
) -> Result<Song, MyError> {
    let song = Song {
        srcdir: song_srcdir
            .parent()
            .ok_or(MyError::MessageError("bad srcdir, no parent".to_string()))?
            .to_str()
            .ok_or(MyError::MessageError("bad srcdir".to_string()))?
            .to_string(),
        texfiles: uconf.texfiles.clone(),
        author: uconf.author.clone(),
        tempo: uconf.tempo,
        time_signature: time_signature_of_string(uconf.time_signature)?,
        pdfname: normalize_pdf_name(&uconf.author, &uconf.title),
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
) -> Result<(Song, UserSongWithPath), MyError> {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .clone();
    let uconf: UserSong = serde_json::from_str(&contents)?;

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
    let song = song_of_usersong(uconf.clone(), filepath.clone(), song_builddir)?;
    let usersongwithpath = UserSongWithPath {
        song: uconf.clone(),
        path: filepath.to_str().unwrap().to_string(),
    };
    check_section_types(&sections, &song)?;
    Ok((song, usersongwithpath))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{input_model, model};

    #[test]
    fn test1_bar_of_string() {
        let ret = row_of_string(1, "A|B|C|D".to_string());
        assert_eq!(
            ret,
            (
                5,
                Row {
                    repeat: 1,
                    row_start_bar_number: 1,
                    bars: vec![
                        Bar {
                            chords: vec!["A".to_string()],
                            time_signature: None
                        },
                        Bar {
                            chords: vec!["B".to_string()],
                            time_signature: None
                        },
                        Bar {
                            chords: vec!["C".to_string()],
                            time_signature: None
                        },
                        Bar {
                            chords: vec!["D".to_string()],
                            time_signature: None
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
                section_body: None,
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
                        repeat: 1,
                        row_start_bar_number: 5,
                        bars: vec![
                            Bar {
                                chords: vec!["A".to_string()],
                                time_signature: None
                            },
                            Bar {
                                chords: vec!["B".to_string()],
                                time_signature: None
                            },
                            Bar {
                                chords: vec!["C".to_string()],
                                time_signature: None
                            },
                            Bar {
                                chords: vec!["D".to_string()],
                                time_signature: None
                            }
                        ]
                    },
                    Row {
                        repeat: 1,
                        row_start_bar_number: 9,
                        bars: vec![Bar {
                            chords: vec!["Gf".to_string()],
                            time_signature: None
                        }]
                    },
                    Row {
                        repeat: 1,
                        row_start_bar_number: 10,
                        bars: vec![
                            Bar {
                                chords: vec!["C".to_string()],
                                time_signature: None
                            },
                            Bar {
                                chords: vec!["D".to_string()],
                                time_signature: None
                            },
                            Bar {
                                chords: vec!["C".to_string()],
                                time_signature: None
                            },
                            Bar {
                                chords: vec!["D".to_string()],
                                time_signature: None
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
                section_body: None,
                rows: vec![
                    "Af Bfm7 | E E ".to_string(),
                    "B".to_string(),
                    "C|D".to_string(),
                ],
            }),
            id: "".to_string(),
        };
        let expected = StructureItem {
            item: StructureItemContent::ItemChords(model::Chords {
                section_title: "".to_string(),
                section_id: "".to_string(),
                section_type: "".to_string(),
                section_body: "".to_string(),
                nbcols: 2,
                nbrows: 3,
                row_start_bar_number: 10,
                nb_bars: 5,
                rows: vec![
                    Row {
                        repeat: 1,
                        row_start_bar_number: 10,
                        bars: vec![
                            Bar {
                                chords: vec!["Af".to_string(), "Bfmsept".to_string()],
                                time_signature: None,
                            },
                            Bar {
                                chords: vec!["E".to_string(), "E".to_string()],
                                time_signature: None,
                            },
                        ],
                    },
                    Row {
                        repeat: 1,
                        row_start_bar_number: 12,
                        bars: vec![Bar {
                            chords: vec!["B".to_string()],
                            time_signature: None,
                        }],
                    },
                    Row {
                        repeat: 1,
                        row_start_bar_number: 13,
                        bars: vec![
                            Bar {
                                chords: vec!["C".to_string()],
                                time_signature: None,
                            },
                            Bar {
                                chords: vec!["D".to_string()],
                                time_signature: None,
                            },
                        ],
                    },
                ],
            }),
        };
        assert_eq!((15, expected), structure_of_structure(10, &vec![], &u));
    }

    #[test]
    fn test_4() {
        let input = UserSong {
            tempo: 80,
            time_signature: None,
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
                        section_body: None,
                        rows: vec![
                            "A|B|C|time_signature(2/4)D|x3".to_string(),
                            "E|F|G|A".to_string(),
                        ],
                    }),
                    id: "blahblah".to_string(),
                },
                input_model::UserStructureItem {
                    item: UserStructureItemContent::Ref(input_model::UserRef {
                        section_title: "".to_string(),
                        link: "blahblah".to_string(),
                        section_body: None,
                        section_type: None,
                    }),
                    id: "".to_string(),
                },
                input_model::UserStructureItem {
                    item: UserStructureItemContent::Ref(input_model::UserRef {
                        section_title: "".to_string(),
                        section_body: None,
                        link: "blahblah".to_string(),
                        section_type: None,
                    }),
                    id: "".to_string(),
                },
            ],
        };
        let output =
            song_of_usersong(input, PathBuf::from("/blah/x.json"), PathBuf::new()).unwrap();
        let expected = Song {
            srcdir: "/blah".to_string(),
            title: "".to_string(),
            author: "".to_string(),
            tempo: 80 as u32,
            time_signature: TimeSignature { top: 4, low: 4 },
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
                        section_body: "".to_string(),
                        row_start_bar_number: 1,
                        nb_bars: 16,
                        nbcols: 4,
                        nbrows: 2,
                        rows: vec![
                            Row {
                                repeat: 3,
                                row_start_bar_number: 1,
                                bars: vec![
                                    Bar {
                                        chords: vec!["A".to_string()],
                                        time_signature: None,
                                    },
                                    Bar {
                                        chords: vec!["B".to_string()],
                                        time_signature: None,
                                    },
                                    Bar {
                                        chords: vec!["C".to_string()],
                                        time_signature: None,
                                    },
                                    Bar {
                                        chords: vec!["D".to_string()],
                                        time_signature: Some(TimeSignature { top: 2, low: 4 }),
                                    },
                                ],
                            },
                            Row {
                                repeat: 1,
                                row_start_bar_number: 13,
                                bars: vec![
                                    Bar {
                                        chords: vec!["E".to_string()],
                                        time_signature: None,
                                    },
                                    Bar {
                                        chords: vec!["F".to_string()],
                                        time_signature: None,
                                    },
                                    Bar {
                                        chords: vec!["G".to_string()],
                                        time_signature: None,
                                    },
                                    Bar {
                                        chords: vec!["A".to_string()],
                                        time_signature: None,
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
                        section_body: "".to_string(),
                        row_start_bar_number: 17,
                        nb_bars: 16,
                    }),
                },
                model::StructureItem {
                    item: model::StructureItemContent::ItemRef(model::Ref {
                        section_title: "".to_string(),
                        section_id: "".to_string(),
                        section_type: "".to_string(),
                        section_body: "".to_string(),
                        row_start_bar_number: 33,
                        nb_bars: 16,
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

    #[test]
    fn test_5() {
        let x = "E".to_string();
        let expected = Bar {
            chords: vec!["E".to_string()],
            time_signature: None,
        };
        let result = bar_of_string(x).unwrap();
        assert_eq!(expected, result);

        let x = "{latex \\tiny{uu}}".to_string();
        let expected = Bar {
            chords: vec!["\\tiny{uu}".to_string()],
            time_signature: None,
        };
        let result = bar_of_string(x).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_6() {
        let x = "time_signature(2/4)E".to_string();
        let expected = Bar {
            chords: vec!["E".to_string()],
            time_signature: Some(TimeSignature { top: 2, low: 4 }),
        };
        let result = bar_of_string(x).unwrap();
        assert_eq!(expected, result);

        let x = "time_signature(3/4) {latex \\tiny{uu}}".to_string();
        let expected = Bar {
            chords: vec!["\\tiny{uu}".to_string()],
            time_signature: Some(TimeSignature { top: 3, low: 4 }),
        };
        let result = bar_of_string(x).unwrap();
        assert_eq!(expected, result);
    }
}
