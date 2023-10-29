//use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::config::model::UserSong;
use crate::config::model::{Bar, Row, Section, Song, UserSection};

pub fn normalize(input: &String) -> String {
    let mut output = input.clone();
    output.make_ascii_lowercase();
    output = output
        .replace(" ", "_")
        .replace("/", "_")
        .replace(".", "_")
        .replace("'", "_");
    output
}

fn bar_of_string(s: &String) -> Bar {
    let chords: Vec<String> = s.rsplit(' ').map(|s| s.to_string()).collect();
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
    let uconf: UserSong = serde_yaml::from_str(&contents).expect("read yml");
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
        sections: uconf.sections.iter().map(section_of_usection).collect(),
        chord_glyph_scale: uconf.chord_glyph_scale.unwrap_or(2),
        outputformat: uconf.outputformat.unwrap_or("mps".to_string()),
        outputtemplate: uconf
            .outputtemplate
            .unwrap_or("mps/main-%c.mps".to_string()),
        section_spacing: uconf.section_spacing.unwrap_or(20),
    };
    Ok(song)
}
