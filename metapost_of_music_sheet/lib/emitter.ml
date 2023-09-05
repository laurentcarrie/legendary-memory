open Printf
module Log = Dolog.Log

let clean_string data =
  let data = Str.global_replace (Str.regexp_string "&amp;quot;") "\"" data in
  let data = Str.global_replace (Str.regexp_string "&quot;") "\"" data in
  let data = Str.global_replace (Str.regexp_string "&gt;") ">" data in
  let data = Str.global_replace (Str.regexp_string "&amp;") "&" data in
  data

let sheet_jingoo : string =
  {whatever|

prologues:=3;
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




% YYYYYYYYYYYYYYYYYYYYYYYY


% -- vardef draw_bati
{{vardef_make_draw_bati}}


% -- vardef make_flat
{{vardef_make_flat}}

% -- vardef make_sharp
{{vardef_make_sharp}}

% -- vardef make_major_seven
{{vardef_make_major_seven}}

% -- vardef make_seven
{{vardef_make_seven}}

% -- vardef make_minor
{{vardef_make_minor}}


% -- vardef glyph_of_chord
{{vardef_make_glyph_of_chord}}

% -- vardef draw_chord
{{vardef_make_draw_chord}}

% -- vardef draw_row
{{vardef_make_draw_row}}



beginfig(0);
    u:=.2cm ;
    margin:=4cm ;
    path p ;
    p := (-margin,-margin) -- (-margin,margin) -- (margin,margin) --
    (margin,-margin)  -- cycle ;
    color background ;
    background := (.8,.7,.7) ;
    %fill p withcolor background ;
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

%    draw_chord("A",A,background) ;
%    draw_chord("B",A shifted(width,0),background) ;

    {{ after_sections }}

endfig;

{{ other }}

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
    (*    let () = Log.info "result_rows : '%s'" (List.hd result_rows) in *)
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
    (*    let _ = Log.info "sections : %s" sections in *)
    (*    let _ = Log.info "XXXXXXXXXXXXXXXXXXXX %s" Mp_code.make_flat in *)
    let _ = sections in
    Jingoo.Jg_template.from_string sheet_jingoo
      ~models:
        [
          ("width", Jingoo.Jg_types.Tint 1);
          ("height", Jingoo.Jg_types.Tint 1);
          ("section_spacing", Jingoo.Jg_types.Tint 20);
          ("outputtemplate", Jingoo.Jg_types.Tstr "mps/main-%c.mps");
          ("outputformat", Jingoo.Jg_types.Tstr "mps");
          ("sections", Jingoo.Jg_types.Tstr sections);
          ("after_sections", Jingoo.Jg_types.Tstr "");
          ("other", Jingoo.Jg_types.Tstr "");
          ("vardef_make_flat", Jingoo.Jg_types.Tstr Mp_code.make_flat);
          ("vardef_make_sharp", Jingoo.Jg_types.Tstr Mp_code.make_sharp);
          ("vardef_make_draw_row", Jingoo.Jg_types.Tstr Mp_code.make_draw_row);
          ( "vardef_make_draw_chord",
            Jingoo.Jg_types.Tstr Mp_code.make_draw_chord );
          ( "vardef_make_glyph_of_chord",
            Jingoo.Jg_types.Tstr Mp_code.make_glyph_of_chord );
          ("vardef_make_seven", Jingoo.Jg_types.Tstr Mp_code.make_seven);
          ( "vardef_make_major_seven",
            Jingoo.Jg_types.Tstr Mp_code.make_major_seven );
          ("vardef_make_minor", Jingoo.Jg_types.Tstr Mp_code.make_minor);
          ("vardef_make_draw_bati", Jingoo.Jg_types.Tstr Mp_code.make_draw_bati);
        ]
  in
  let result = clean_string (emit_sheet sheet) in
  fprintf fout "%s" result
