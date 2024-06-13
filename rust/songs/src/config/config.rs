//use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::config::model::UserSong;
use crate::config::model::{Bar, Row, Section, Song, UserSection};

fn bar_of_string(s: &String) -> Bar {
    let chords: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
    //.iter().map(|s| s.to_string()).collect() ;
    Bar { chords: chords }
}

fn row_of_vstring(v: &Vec<String>) -> Row {
    let bars: Vec<Bar> = v.iter().map(bar_of_string).collect();
    Row { bars: bars }
}

fn section_of_usection(u: &UserSection) -> Section {
    // let mut split = "Mary had a little lamb".split_whitespace();
    let rows: Vec<Row> = u.rows.iter().map(row_of_vstring).collect();
    return Section {
        name: u.name.clone(),
        rows: rows,
    };
}

pub fn decode_song(buildroot: &PathBuf, filepath: &PathBuf) -> Result<Song, Error> {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .clone();
    //println!("{}", contents);
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
        cell_height: uconf.cell_height,
        cell_width: uconf.cell_width,
        texfiles: uconf.texfiles,
        author: uconf.author,
        title: uconf.title,
        builddir: song_builddir,
        lilypondfiles: uconf.lilypondfiles,
        wavfiles: uconf.wavfiles,
        sections: uconf.sections.iter().map(section_of_usection).collect(),
        chord_glyph_scale: uconf.chord_glyph_scale.unwrap_or(2),
        outputformat: uconf.outputformat.unwrap_or("mps".to_string()),
        outputtemplate: uconf.outputtemplate.unwrap_or("mps/%s-%c.mps".to_string()),
        section_spacing: uconf.section_spacing.unwrap_or(20),
        date: uconf.date,
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
