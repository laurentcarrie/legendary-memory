open Printf
module Log = Dolog.Log

let sheet_jingoo : string =
  {whatever|

prologues:=3;
% outputtemplate := "mps/frame_%c.{{format}}";
outputtemplate := "{{outputtemplate}}";
outputformat := "{{format}}";
input boxes ;
input TEX ;
verbatimtex
\documentclass{article}
%%\usepackage{lmodern}
\usepackage[tt=false]{libertine}
\usepackage[libertine]{newtxmath}
\usepackage{amsmath}
\begin{document}
etex

fontmapfile "=lm-ec.map";


vardef glyph_of_chord (expr chord)=
    save p ;
    picture p ;
    if     chord="A": p:=glyph "A" of "ec-lmr10" ;
    elseif chord="B": p:=glyph "B" of "ec-lmr10" ;
    elseif chord="C": p:=glyph "C" of "ec-lmr10" ;
    elseif chord="D": p:=glyph "D" of "ec-lmr10" ;
    elseif chord="E": p:=glyph "E" of "ec-lmr10" ;
    elseif chord="F": p:=glyph "F" of "ec-lmr10" ;
    elseif chord="G": p:=glyph "G" of "ec-lmr10" ;
    else: p:=glyph "X" of "ec-lmr10" ; fi;

    p
enddef ;


vardef draw_chord(expr chord,S,background) =
    save q,p ;
    picture q;
    path p;
    interim ahlength := 12bp;
    interim ahangle := 25;
    q := glyph_of_chord (chord) ;
    q := q scaled .01 ;
    q := q shifted ( S - center bbox q ) ;
    for item within q:
        p := pathpart item ;
        if turningnumber p = 1:
            fill p withcolor red ;
        else:
            fill p withcolor background ;
        fi;
        for j=0 upto length p:
            pickup pencircle scaled .01;
            %draw (point j of p -- precontrol j of p)   dashed evenly withcolor blue;
            %draw (point j of p -- postcontrol j of p)  dashed evenly withcolor blue;
            pickup pencircle scaled .03;
            %draw precontrol j of p withcolor red;
            %draw postcontrol j of p withcolor red;
            pickup pencircle scaled .02;
            %draw point j of p withcolor black;
        endfor ;
    endfor ;

enddef ;


vardef draw_row(suffix B)(expr A,width,height,n,background)(suffix chords) =
    save chord ;
    color c ;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*width,0) ;
    B2 := B1 shifted (0,-height) ;
    B3 := A shifted (0,-height) ;
    draw B0 -- B1 -- B2 -- B3 -- cycle withcolor c ;

    for i=1 step 1 until n :
        draw B0 shifted (i*width,0) -- B3 shifted (i*width,0) withcolor c ;
    endfor ;

    for i=0 step 1 until n-1:
        pair box[] ;
        box0 = B0 shifted (i*width,0) ;
        box1 = box0 shifted (width,0) ;
        box2 = box1 shifted (0,-height) ;
        box3 = box0 shifted (0,-height) ;
        box4 = .5[box0,box2] ;
        pair S ;
        S = .5(box0+box2) ;
        string chord ;
        chord := chords[i] ;
        show(chord) ;
        draw_chord(chord,S,background) ;
    endfor ;

enddef ;
beginfig(0);
    u:=.2cm ;
    margin:=4cm ;
    path p ;
    p := (-margin,-margin) -- (-margin,margin) -- (margin,margin) --
    (margin,-margin)  -- cycle ;
    color background ;
    background := (.8,.7,.7) ;
    fill p withcolor background ;
    %label(decimal t,(-margin,-margin)/2) ;
    %%draw textext("cycle " & decimal t) shifted (-margin,-margin)/2  ;

    numeric n,width,height ;
    % n := {{n}} ;
    pair A ;
    %width := {{width}} ;
    %height := {{height}} ;

    {{ sections }}

    %A = (-3cm,3cm) shifted (0,{{counter_row}}*-3cm);
    %string chords[] ;
endfig;
end.
|whatever}

let section_jingoo : string =
  {whatever|
% section {{section.name}}
    {% for c in row %}
    chords{{loop.index0}} := "{{c}}" ;     {% endfor %}
    pair B[] ;
|whatever}

let emit fout sheet format outputtemplate =
  let _ = format in
  let _ = outputtemplate in

  (*  let extension = match format with *)
  (*  | "png" -> "png" *)
  (*  |"mp" -> "mp" *)
  (*  |_-> failwith "bad format" *)
  (*  in *)

  (*    let range from until = *)
  (*        List.init (until - from) (fun i -> Jingoo.Jg_types.Tint (i + from)) *)
  (*    in *)
  (*  let emit_row row counter_row = *)
  (*    let result : string = *)
  (*      Jingoo.Jg_template.from_string sheet_jingoo *)
  (*        ~models: *)
  (*          [ *)
  (*            ("format", Jingoo.Jg_types.Tstr format); *)
  (*            ("outputtemplate", Jingoo.Jg_types.Tstr outputtemplate); *)
  (*            ("width", Jingoo.Jg_types.Tstr "1cm"); *)
  (*            ("height", Jingoo.Jg_types.Tstr ".5cm"); *)
  (*            ("n", Jingoo.Jg_types.Tint (List.length row)); *)
  (*            ("counter_row", Jingoo.Jg_types.Tint counter_row); *)
  (*            ( "row", *)
  (*              let f c = Jingoo.Jg_types.Tstr c in *)
  (*              Jingoo.Jg_types.Tlist (List.map f row) ); *)
  (*          ] *)
  (*    in *)
  (*    result *)
  (*  in *)
  let emit_row _ = "% row\n " in
  let emit_section section =
    let result = "% SECTION " ^ section.Sheet.name ^ "\n" in
    let rows : string =
      List.fold_left (fun acc row -> acc ^ emit_row row) "" section.Sheet.rows
    in
    let result = result ^ rows in
    result
  in

  let emit_sheet sheet =
    let sections : string =
      List.fold_left
        (fun acc section -> acc ^ emit_section section)
        "" sheet.Sheet.sections
    in
    let _ = Log.info "sections : %s" sections in
    let result =
      Jingoo.Jg_template.from_string sheet_jingoo
        ~models:
          [
            ("format", Jingoo.Jg_types.Tstr format);
            ("outputtemplate", Jingoo.Jg_types.Tstr outputtemplate);
            ("width", Jingoo.Jg_types.Tstr "1cm");
            ("sections", Jingoo.Jg_types.Tstr sections);
          ]
    in
    result
  in
  let result = emit_sheet sheet in
  fprintf fout "%s" result
