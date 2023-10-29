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
    writeln!(output, "n:={length} ;", length = row.bars.len())?;
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
draw_row(A,cell_width,cell_height,n,background)(chords,barindex,nbchordsinbar,indexinbar) ;
A := A shifted (0,-cell_height) ;
"
    )?;
    Ok(())
}
// {% for chord in chords %}
// chords{{loop.index0}}:="{{chord}}" ;
// {% endfor %}
// {% for i in barindex %}
// barindex{{loop.index0}}:={{i}} ;
// {% endfor %}
// {% for i in nbchordsinbar %}
// nbchordsinbar{{loop.index0}}:={{i}} ;
// {% endfor %}
// {% for i in indexinbar %}
// indexinbar{{loop.index0}}:={{i}} ;
// {% endfor %}
//
// draw_row(A,cell_width,cell_height,n,background)(chords,barindex,nbchordsinbar,indexinbar) ;
// A := A shifted (0,-cell_height) ;
// |whatever}
//

fn write_section(mut output: &File, section: &Section) -> Result<(), Error> {
    writeln!(
        output,
        "{}",
        format!(
            r###"
% SECTION {name}
A := A shifted (0,-section_spacing) ;
%draw fullcircle scaled 2 shifted A withcolor red ;
%label.urt(btex {name}   etex,A) ;
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

pub fn write_mp(song: &Song) -> Result<(), Error> {
    let data = format!(
        r###"
prologues:=3;
outputtemplate := "{outputtemplate}";
outputformat := "{outputformat}";
input boxes ;
input TEX ;
%verbatimtex \nofiles etex;
verbatimtex
\documentclass{{ article }}
%%\usepackage{{lmodern}}
\usepackage[tt=false]{{libertine}}
\usepackage[libertine]{{newtxmath}}
\usepackage{{amsmath}}
\begin{{document}}
etex

%fontmapfile "=lm-ec.map";


numeric  chord_glyph_scale ;
chord_glyph_scale:={chord_glyph_scale} ;


% YYYYYYYYYYYYYYYYYYYYYYYY


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
        outputtemplate = song.outputtemplate,
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

    let mut p: PathBuf = song.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("main.mp");
    println!("write {}", p.display());
    let mut output = File::create(p)?;
    writeln!(output, "{}", &data)?;
    for section in &song.sections {
        write_section(&output, &section)?;
    }

    //    {{ after_sections }}

    writeln!(output, "endfig;")?;

    //{{ other }}

    writeln!(output, "end.")?;
    writeln!(output, "")?;

    Ok(())
}

// let row_jingoo : string =
// {whatever|
// % row
// n:={{n}} ;
// {% for chord in chords %}
// chords{{loop.index0}}:="{{chord}}" ;
// {% endfor %}
// {% for i in barindex %}
// barindex{{loop.index0}}:={{i}} ;
// {% endfor %}
// {% for i in nbchordsinbar %}
// nbchordsinbar{{loop.index0}}:={{i}} ;
// {% endfor %}
// {% for i in indexinbar %}
// indexinbar{{loop.index0}}:={{i}} ;
// {% endfor %}
//
// draw_row(A,cell_width,cell_height,n,background)(chords,barindex,nbchordsinbar,indexinbar) ;
// A := A shifted (0,-cell_height) ;
// |whatever}
//
// let section_jingoo : string =
// {whatever|
// % SECTION {{name}}
// A := A shifted (0,-section_spacing) ;
// %draw fullcircle scaled 2 shifted A withcolor red ;
// label.urt(btex {{name}}   etex,A) ;
// %label.ulft(btex \rmfamily \textit{ {{name}} } etex,A) ;
// {% for row in rows %}%{{row}}
// {% endfor %}
// |whatever}
//
// let emit fout sheet format outputtemplate =
// let _ = format in
// let _ = outputtemplate in
// let emit_row row =
// (*    let env = Jingoo.Jg_types.std_env in *)
// (*    let env = { env with autoescape = false } in *)
// let result =
// (* we need the list of chords, with the list of the bar number of each chord *)
// let _, nbchordsinbar, indexinbar, chordlist, barindexlist =
// List.fold_left
// (fun acc bar ->
// let ( barindex,
// current_nbchordsinbar,
// current_indexinbar,
// current_chords,
// current_barindex ) =
// acc
// in
// let added_chords = bar.Sheet.chords in
// let added_barindex =
// List.map (fun _ -> barindex) bar.Sheet.chords
// in
// let added_nbchordsinbar =
// List.map (fun _ -> List.length bar.Sheet.chords) bar.Sheet.chords
// in
// let added_indexinbar = List.mapi (fun i _ -> i) bar.Sheet.chords in
// ( barindex + 1,
// List.concat [ current_nbchordsinbar; added_nbchordsinbar ],
// List.concat [ current_indexinbar; added_indexinbar ],
// List.concat [ current_chords; added_chords ],
// List.concat [ current_barindex; added_barindex ] ))
// (0, [], [], [], []) row.Sheet.bars
// in
// Log.debug "%d %d %d %d"
// (List.length nbchordsinbar)
// (List.length indexinbar) (List.length chordlist)
// (List.length barindexlist);
//
// let si = SetInt.empty in
// let si =
// List.fold_right SetInt.add
// [
// List.length nbchordsinbar;
// List.length indexinbar;
// List.length chordlist;
// List.length barindexlist;
// ]
// si
// in
// SetInt.iter (fun value -> Log.debug "-----------------> %d" value) si;
// if SetInt.cardinal si != 1 then failwith "internal error";
//
// List.iter
// (fun (i, c) -> Log.debug "bar %d ; chord %s" i c)
// (List.combine barindexlist chordlist);
//
// List.iter (fun i -> Log.debug "indexinbar %d" i) indexinbar;
//
// Jingoo.Jg_template.from_string row_jingoo (*      ~env:env *)
// ~models:
// [
// ("n", Jingoo.Jg_types.Tint (List.length chordlist));
// ( "chords",
// Jingoo.Jg_types.Tlist
// (List.map
// (fun s ->
// Jingoo.Jg_types.Tstr (Jingoo.Jg_utils.escape_html s))
// chordlist) );
// ( "barindex",
// Jingoo.Jg_types.Tlist
// (List.map (fun s -> Jingoo.Jg_types.Tint s) barindexlist) );
// ( "nbchordsinbar",
// Jingoo.Jg_types.Tlist
// (List.map (fun s -> Jingoo.Jg_types.Tint s) nbchordsinbar) );
// ( "indexinbar",
// Jingoo.Jg_types.Tlist
// (List.map (fun s -> Jingoo.Jg_types.Tint s) indexinbar) );
// ]
// in
// let result = clean_string result in
// result
// in
//
// let emit_section section =
// let result_rows = List.map emit_row section.Sheet.rows in
// (*    let () = Log.debug "result_rows : '%s'" (List.hd result_rows) in *)
// let result =
// Jingoo.Jg_template.from_string section_jingoo
// ~models:
// [
// ("name", Jingoo.Jg_types.Tstr section.Sheet.name);
// ( "rows",
// Jingoo.Jg_types.Tlist
// (List.map (fun s -> Jingoo.Jg_types.Tstr s) result_rows) );
// ]
// in
// result
// in
//
// let emit_sheet sheet =
// let sections : string =
// List.fold_left
// (fun acc section -> acc ^ emit_section section)
// "" sheet.Sheet.sections
// in
// (*    let _ = Log.debug "sections : %s" sections in *)
// (*    let _ = Log.debug "XXXXXXXXXXXXXXXXXXXX %s" Mp_code.make_flat in *)
// let _ = sections in
// Jingoo.Jg_template.from_string sheet_jingoo
// ~models:
// [
// ("cell_width", Jingoo.Jg_types.Tfloat sheet.Sheet.cell_width);
// ("cell_height", Jingoo.Jg_types.Tfloat sheet.Sheet.cell_height);
// ( "chord_glyph_scale",
// Jingoo.Jg_types.Tfloat sheet.Sheet.chord_glyph_scale );
// ("section_spacing", Jingoo.Jg_types.Tint 20);
// ("outputtemplate", Jingoo.Jg_types.Tstr "mps/main-%c.mps");
// ("outputformat", Jingoo.Jg_types.Tstr "mps");
// ("sections", Jingoo.Jg_types.Tstr sections);
// ("after_sections", Jingoo.Jg_types.Tstr "");
// ("other", Jingoo.Jg_types.Tstr "");
// ("vardef_make_flat", Jingoo.Jg_types.Tstr Mp_code.make_flat);
// ("vardef_make_sharp", Jingoo.Jg_types.Tstr Mp_code.make_sharp);
// ("vardef_make_draw_row", Jingoo.Jg_types.Tstr Mp_code.make_draw_row);
// ( "vardef_make_draw_chord",
// Jingoo.Jg_types.Tstr Mp_code.make_draw_chord );
// ( "vardef_make_glyph_of_chord",
// Jingoo.Jg_types.Tstr Mp_code.make_glyph_of_chord );
// ("vardef_make_seven", Jingoo.Jg_types.Tstr Mp_code.make_seven);
// ( "vardef_make_major_seven",
// Jingoo.Jg_types.Tstr Mp_code.make_major_seven );
// ("vardef_make_minor", Jingoo.Jg_types.Tstr Mp_code.make_minor);
// ("vardef_make_draw_bati", Jingoo.Jg_types.Tstr Mp_code.make_draw_bati);
// ]
// in
// let result = clean_string (emit_sheet sheet) in
// fprintf fout "%s" result
