open Printf
module Log = Dolog.Log

let clean_string data =
  let data = Str.global_replace (Str.regexp_string "&amp;quot;") "\"" data in
  let data = Str.global_replace (Str.regexp_string "&quot;") "\"" data in
  data

let sheet_jingoo : string =
  {whatever|

YYY
{{vardefmakeflat}}
"{{format}}"
YYYx
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

% -- vardef make_flat
{{vardef_make_flat}}

% -- vardef make_sharp
{{vardef_make_sharp}}



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
    let _ = Log.info "XXXXXXXXXXXXXXXXXXXX %s" Mp_code.make_flat in
    Jingoo.Jg_template.from_string sheet_jingoo
      ~models:
        [
          ("vardefmakeflat", Jingoo.Jg_types.Tstr "ZZZZZZZZZZZZZZZZZZZZZ");
          ("format", Jingoo.Jg_types.Tstr format);
          ("outputtemplate", Jingoo.Jg_types.Tstr outputtemplate);
          ("width", Jingoo.Jg_types.Tstr "1cm");
          ("height", Jingoo.Jg_types.Tstr ".3cm");
          ("section_spacing", Jingoo.Jg_types.Tstr ".5cm");
          ("sections", Jingoo.Jg_types.Tstr sections);
          ("after_sections", Jingoo.Jg_types.Tstr "");
          ("other", Jingoo.Jg_types.Tstr "");
          ("vardef_make_sharp", Jingoo.Jg_types.Tstr Mp_code.make_sharp);
        ]
  in
  let result = clean_string (emit_sheet sheet) in
  fprintf fout "%s" result
