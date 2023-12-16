// open Printf
// module Log = Dolog.Log
// module SetInt = Set.Make (Int)
//
// let clean_string data =
// let data = Str.global_replace (Str.regexp_string "&amp;lt;") "<" data in
// let data = Str.global_replace (Str.regexp_string "&amp;gt;") ">" data in
// let data = Str.global_replace (Str.regexp_string "&amp;quot;") "\"" data in
// let data = Str.global_replace (Str.regexp_string "&quot;") "\"" data in
// let data = Str.global_replace (Str.regexp_string "&gt;") ">" data in
// let data = Str.global_replace (Str.regexp_string "&amp;") "&" data in
// data

use crate::config::model::{Row, Section, Song};
use crate::generated::mp_code::{
    make_draw_bati, make_draw_chord, make_draw_row, make_flat, make_glyph_of_chord,
    make_major_seven, make_minor, make_seven, make_sharp,
};
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

fn write_row(mut output: &File, row: &Row) -> Result<(), Error> {
    writeln!(output, "% write row from emitter.rs")?;
    let nbchords: usize = row
        .bars
        .iter()
        .map(|b| b.chords.len())
        .fold(0, |acc, i| acc + i);

    // for b in &row.bars {
    //     println!("---> {n}", n = b.chords.len());
    // }
    // println!("sum is {nbchords}", nbchords = nbchords);

    writeln!(output, "nbbars:={nbbars} ;", nbbars = row.bars.len())?;
    writeln!(output, "nbchords:={nbchords} ;", nbchords = nbchords)?;
    let mut counter_chord = 0;
    let mut counter_bar = 0;
    for bar in &row.bars {
        let mut counter_inbar = 0;
        for chord in &bar.chords {
            writeln!(
                output,
                "chords{counter_chord}:=\"{chord}\" ;",
                counter_chord = counter_chord,
                chord = chord
            )?;
            writeln!(
                output,
                "barindex{counter_chord}:={counter_bar} ;",
                counter_chord = counter_chord,
                counter_bar = counter_bar
            )?;
            writeln!(
                output,
                "nbchordsinbar{counter_chord}:={len} ;",
                counter_chord = counter_chord,
                len = bar.chords.len()
            )?;
            writeln!(
                output,
                "indexinbar{counter_chord}:={counter_inbar} ;",
                counter_chord = counter_chord,
                counter_inbar = counter_inbar
            )?;
            counter_chord += 1;
            counter_inbar += 1;
        }
        counter_bar += 1;
    }
    writeln!(
        output,
        "
draw_row(A,cell_width,cell_height,nbbars,nbchords,background)(chords,barindex,nbchordsinbar,indexinbar) ;
A := A shifted (0,-cell_height) ;
"
    )?;
    Ok(())
}

fn write_section(mut output: &File, section: &Section) -> Result<(), Error> {
    writeln!(
        output,
        "{}",
        format!(
            r###"
% SECTION {name}
A := A shifted (0,-section_spacing) ;
%draw fullcircle scaled 2 shifted A withcolor red ;
label.urt(btex {name}   etex,A) ;
%label.ulft(btex \rmfamily \textit{{{name}}} etex,A) ;
"###,
            name = section.name
        )
    )?;
    for row in &section.rows {
        write_row(&output, &row)?;
    }
    Ok(())
}

pub fn write_mp(section: &Section, song: &Song) -> Result<(), Error> {
    let data = format!(
        r###"
prologues:=3;
outputtemplate := "mps/{section_name}-%c.mps";
outputformat := "{outputformat}";
input boxes ;


verbatimtex
%&latex
\documentclass{{minimal}}
\begin{{document}}
etex


numeric  chord_glyph_scale ;
chord_glyph_scale:={chord_glyph_scale} ;

% -- vardef draw_bati
{vardef_make_draw_bati}

% -- vardef make_flat
{vardef_make_flat}

% -- vardef make_sharp
{vardef_make_sharp}

% -- vardef make_major_seven
{vardef_make_major_seven}

% -- vardef make_seven
{vardef_make_seven}

% -- vardef make_minor
{vardef_make_minor}


% -- vardef glyph_of_chord
{vardef_make_glyph_of_chord}

% -- vardef draw_chord
{vardef_make_draw_chord}

% -- vardef draw_row
{vardef_make_draw_row}



beginfig(0);
u:=.2cm ;
margin:=4cm ;
path p ;
%    p := (-margin,-margin) -- (-margin,margin) -- (margin,margin) --
%    (margin,-margin)  -- cycle ;
%    p := (-margin,-margin) -- (-margin,margin) -- (margin,margin) --
%    (margin,-margin)  -- cycle ;
color background ;
%background := (.8,.7,.7) ;
background := (1,1,1) ;
%    fill p withcolor background ;
%label(decimal t,(-margin,-margin)/2) ;
%%draw textext("cycle " & decimal t) shifted (-margin,-margin)/2  ;

%label(textext("Pythagorean addition: $a^2+b^2 = c^2$."), origin);
%label(btex \rmfamily Pythagorean addition : $a$ etex, origin);
%label(btex \sffamily Pythagorean addition : $a$ etex, origin shifted (0,-1cm));

numeric n,cell_width,cell_height ;
pair A,B[] ;
B0=origin ;
B1=origin ;
B2=origin ;
B3=origin ;
B4=origin ;
string chords[] ;
A := (0cm,0cm) ;
cell_width := {cell_width} ;
cell_height :=  {cell_height} ;
section_spacing :=  {section_spacing} ;


"###,
        section_name = section.name,
        outputformat = song.outputformat,
        chord_glyph_scale = song.chord_glyph_scale,
        vardef_make_flat = make_flat(),
        vardef_make_sharp = make_sharp(),
        vardef_make_draw_bati = make_draw_bati(),
        vardef_make_major_seven = make_major_seven(),
        vardef_make_seven = make_seven(),
        vardef_make_minor = make_minor(),
        vardef_make_glyph_of_chord = make_glyph_of_chord(),
        vardef_make_draw_chord = make_draw_chord(),
        vardef_make_draw_row = make_draw_row(),
        cell_width = song.cell_width,
        cell_height = song.cell_height,
        section_spacing = song.section_spacing,
    );

    // for (index, section) in song.sections.iter().enumerate() {
    let mut p: PathBuf = song.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push(format!("{name}.mp", name = section.name));
    println!("write {}", p.display());
    let mut output = File::create(p)?;
    writeln!(output, "{}", &data)?;
    // println!("INDEX {}", index);
    write_section(&output, &section)?;
    //    {{ after_sections }}

    writeln!(output, "endfig;")?;

    //{{ other }}

    writeln!(output, "end.")?;
    writeln!(output, "")?;
    // }

    Ok(())
}
