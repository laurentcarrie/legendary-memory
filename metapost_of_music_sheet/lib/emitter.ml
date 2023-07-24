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

%fontmapfile "=lm-ec.map";


vardef base_glyph_of_chord (expr chord)=
    save p ;
    picture p ;
    string font ;
    %font = "ec-lmr10" ;
    %font = "t5-qcrri" ;
    font := "eurm10";
    if     chord="A": p:=glyph "A" of font ;
    elseif chord="B": p:=glyph "B" of font ;
    elseif chord="C": p:=glyph "C" of font ;
    elseif chord="D": p:=glyph "D" of font ;
    elseif chord="E": p:=glyph "E" of font ;
    elseif chord="F": p:=glyph "F" of font ;
    elseif chord="G": p:=glyph "G" of font ;
    else:             p:=glyph "X" of font ;
    fi;

    p
enddef ;

vardef make_minor(expr chord)=
    save p ;
    picture p[] ;
    p := base_glyph_of_chord(chord)  ;
    picture p2 ;
    p2 := image (
        draw (0,0) -- (1cm,1cm) withcolor (0,1,0) ;
        ) ;
    addto p also p2 ;

    p
enddef ;

vardef make_major_sept(expr chord)=
    save p ;
    picture p,q ;
    p := base_glyph_of_chord(chord)  ;
    path a ;
    a := (-3cm,-3cm) -- (3cm,3cm) -- cycle ;
    %addto p contour fullcircle scaled .1cm   ;
    p := image(draw fullcircle scaled .01cm withcolor blue) ;
    %draw p ;
    p
enddef ;


vardef glyph_of_chord (expr chord)=
    save p ;
    picture p ;

    if     chord="Am": p:=make_minor("A");
    elseif chord="Bm": p:=make_minor("B");
    elseif chord="Cm": p:=make_minor("C");
    elseif chord="Dm": p:=make_minor("D");
    elseif chord="Em": p:=make_minor("E");
    elseif chord="Fm": p:=make_minor("F");
    elseif chord="Gm": p:=make_minor("G");

    elseif chord="AM7": p:=make_major_sept("A");
    elseif chord="BM7": p:=make_major_sept("B");
    elseif chord="CM7": p:=make_major_sept("C");
    elseif chord="DM7": p:=make_major_sept("D");
    elseif chord="EM7": p:=make_major_sept("E");
    elseif chord="FM7": p:=make_major_sept("F");
    elseif chord="GM7": p:=make_major_sept("G");

    else:               p:=base_glyph_of_chord(chord)  ; fi;

    p
enddef ;


vardef draw_chord(expr chord,S,background) =
    save q,p ;
    picture q;
    path p;
    interim ahlength := 12bp;
    interim ahangle := 25;
    q := glyph_of_chord (chord) ;
    q := q scaled .01 scaled .8 ;
    q := q shifted ( S - center bbox q ) ;
    for item within q:
        p := pathpart item ;
        if turningnumber p = 1:
            fill p withcolor red ;
        else:
            %fill p withcolor background ;
            unfill p ;
        fi;
        for j=0 upto length p:
            %pickup pencircle scaled .01;
            %draw (point j of p -- precontrol j of p)   dashed evenly withcolor blue;
            %draw (point j of p -- postcontrol j of p)  dashed evenly withcolor blue;
            %pickup pencircle scaled .03;
            %draw precontrol j of p withcolor red;
            %draw postcontrol j of p withcolor red;
            %pickup pencircle scaled .02;
            %draw point j of p withcolor black;
        endfor ;
    endfor ;

enddef ;


vardef draw_row(expr A,width,height,n,background)(suffix chords) =
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
        %show(chord) ;
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

    %label(textext("Pythagorean addition: $a^2+b^2 = c^2$."), origin);
    %label(btex \rmfamily Pythagorean addition : $a$ etex, origin);
    %label(btex \sffamily Pythagorean addition : $a$ etex, origin shifted (0,-1cm));

    numeric n,width,height ;
    pair A,B[] ;
    B0=origin ;
    B1=origin ;
    B2=origin ;
    B3=origin ;
    B4=origin ;
    string chords[] ;
    A := (-3cm,3cm) ;
    width := {{width}} ;
    height := {{height}} ;
    section_spacing := {{section_spacing}} ;

    {{ sections }}

    %A = (-3cm,3cm) shifted (0,{{counter_row}}*-3cm);
    %string chords[] ;
endfig;
end.
|whatever}

let row_jingoo : string =
  {whatever|
% row
n:={{n}} ;
{% for chord in chords %}
chords{{loop.index0}}:="{{chord}}" ;
{% endfor %}
draw_row(A,width,height,n,background)(chords) ;
A := A shifted (0,-height) ;
|whatever}

let section_jingoo : string =
{whatever|
% SECTION {{name}}
A := A shifted (0,-section_spacing) ;
label.urt(btex \rmfamily \textit{ {{name}} } etex,A) ;
{% for row in rows %}%{{row}}
{% endfor %}
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
  let emit_row row =
    (*    let env = Jingoo.Jg_types.std_env in *)
    (*    let env = { env with autoescape=false} in *)
    let result =
      Jingoo.Jg_template.from_string row_jingoo (*      ~env:env *)
        ~models:
          [
            ("n", Jingoo.Jg_types.Tint (List.length row));
            ( "chords",
              Jingoo.Jg_types.Tlist
                (List.map
                   (fun s ->
                     Jingoo.Jg_types.Tstr (Jingoo.Jg_utils.escape_html s))
                   row) );
          ]
    in
    let result =
      Str.global_replace (Str.regexp_string "&amp;quot;") "" result
    in
    result
  in

  let emit_section section =
    let result_rows = List.map emit_row section.Sheet.rows in
    let () = Log.info "result_rows : '%s'" (List.hd result_rows) in
    let result =
      Jingoo.Jg_template.from_string section_jingoo
        ~models:
          [
            ("name", Jingoo.Jg_types.Tstr section.Sheet.name);
            ( "rows",
              Jingoo.Jg_types.Tlist
                (List.map (fun s -> Jingoo.Jg_types.Tstr s) result_rows) );
          ]
    in
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
            ("height", Jingoo.Jg_types.Tstr ".3cm");
            ("section_spacing", Jingoo.Jg_types.Tstr ".5cm");
            ("sections", Jingoo.Jg_types.Tstr sections);
          ]
    in
    let result =
      Str.global_replace (Str.regexp_string "&amp;quot;") "\"" result
    in
    let result =
      Str.global_replace (Str.regexp_string "&quot;") "\"" result
      in
    result
  in
  let result = emit_sheet sheet in
  fprintf fout "%s" result
