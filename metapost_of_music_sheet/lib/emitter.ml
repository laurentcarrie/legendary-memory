open Printf
module Log = Dolog.Log

let clean_string data =
  let data = Str.global_replace (Str.regexp_string "&amp;quot;") "\"" data in
  let data = Str.global_replace (Str.regexp_string "&quot;") "\"" data in
  data

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

% freehand_segment and freehand_path
% https://raw.githubusercontent.com/thruston/Drawing-with-Metapost/master/Drawing-with-Metapost.pdf
% chapter 8.4.1
def freehand_segment(expr p) =
    point 0 of p {direction 0 of p rotated (1+.4*normaldeviate)} ..
    point 1 of p {direction 1 of p rotated (1+.4*normaldeviate)}
enddef;

def freehand_path(expr p) =
    freehand_segment(subpath(0,1) of p)
    for i=1 upto length(p)-1:
    & freehand_segment(subpath(i,i+1) of p)
    endfor
    if cycle p: & cycle fi
enddef;

vardef glyph_of_chord (expr chord)=
    save p ;
    picture p ;
    string font ;
    %font = "ec-lmr10" ;
    %font = "t5-qcrri" ;
    font := "eurm10";
    if     substring(0,1) of chord="A": p:=glyph "A" of font ;
    elseif substring(0,1) of chord="B": p:=glyph "B" of font ;
    elseif substring(0,1) of chord="C": p:=glyph "C" of font ;
    elseif substring(0,1) of chord="D": p:=glyph "D" of font ;
    elseif substring(0,1) of chord="E": p:=glyph "E" of font ;
    elseif substring(0,1) of chord="F": p:=glyph "F" of font ;
    elseif substring(0,1) of chord="G": p:=glyph "G" of font ;
    else:             p:=glyph "X" of font ;
    fi;

    p
enddef ;

vardef make_flat(suffix ret)(expr chord)=
    save is_flat;
    boolean is_flat ;

    if (length chord>1) and ( substring(1,2) of chord = "b" ):
        is_flat:=true;
    else:
        is_flat:=false ;
    fi;

    path p ;

    if is_flat:
        numeric u,n ;
        u := 2 ;
        pair a[] ;

        n:=3 ;
        a[0] := (2.8,0.4);
        a[1] := (3,0.4) ;
        a[2] := (3,2.6) ;
        a[3] := (2.8,2.6) ;
        a[4] := (3,.4) ;
        a[5] := (3,2.6) ;

        p := a[0] for i=1 step 1 until n: -- a[i] endfor  ;

        path q ;
        q := a[4] -- a[5] ;

        p := p -- q -- cycle  ;

        p := p scaled u  ;
    else:
        p := fullcircle scaled 0 ;
    fi;
    ret[0] := p ;
enddef;

vardef make_sharp(expr chord)=
    save is_sharp;
    boolean is_sharp ;

    if (length chord>1) and ( substring(1,2) of chord = "#" ):
        is_sharp:=true;
    else:
        is_sharp:=false ;
    fi;
    save p ;
    path p ;

    if is_sharp:
        numeric u ;
        u := .07cm ;

        pair a[] ;

        pair h ; % horizontal
        h := (.5,0) ;
        pair o ; % oblique
        o := h rotated 80 ;
        pair hh,vv ; % small side
        hh := (.2,0) ;
        vv := hh rotated 90 ;
        pair hhh,vvv ; % between bars
        hhh := (.2,0) ;
        vvv := hhh rotated 90 ;

        n:=27 ;

        a0 = (0,0) ;
        a1 = a0 +  h ;
        a2 - a1 = o ;
        a3 = a2 + hh  ;
        a4 = a3 - o ;
        a5 = a4 + hhh ;
        a6 = a5 + o ;
        a7 = a6 + hh ;
        a8 = a7 - o ;
        a9 = a8 + h ;

        a10 = a9 - vv ;
        a16 - a15 = a8-a7 ;

        ypart a11 = ypart a10 = ypart a26 = ypart a27;
        ypart a24 = ypart a25 = ypart a12 = ypart a13;
        ypart a23 = ypart a22 = ypart a19 = ypart a18 = ypart a15 = ypart a14 ;
        ypart a21 = ypart a20 = ypart a17 = ypart a16 ;

        xpart a13 = xpart a14 = xpart a10 ;

        a27-a0 = a23-a24 = -vv ;
        a24 = a27 - vvv ;

        a11 = whatever [a8,a7] ;
        a12 = whatever [a8,a7] ;
        a15 = whatever [a8,a7] ;
        a16 = whatever [a8,a7] ;

        a17 = whatever [a5,a6] ;
        a18 = whatever [a5,a6] ;

        a19 = whatever [a4,a3] ;
        a20 = whatever [a4,a3] ;

        a26 = whatever [a1,a2] ;
        a25 = whatever [a1,a2] ;
        a22 = whatever [a1,a2] ;
        a21 = whatever [a1,a2] ;


        path p ;
        p := a[0] for i=1 step 1 until n: -- a[i] endfor -- cycle ;
        p := p scaled 2 ;
        p := p shifted (6,2) ;

    else:
        p := fullcircle scaled 0 ;
    fi;
    p
enddef;


vardef make_minor(expr chord)=
    save is_minor;
    boolean is_minor ;

    if (length chord>1) and ( substring(1,2) of chord = "m" ):
        is_minor:=true;
    elseif (length chord>1) and ( substring(1,2) of chord = "-" ):
        is_minor:=true ;
    elseif (length chord>2) and ( substring(2,3) of chord = "m" ):
        is_minor:=true;
    elseif (length chord>2) and ( substring(2,3) of chord = "-" ):
        is_minor:=true ;
    else:
        is_minor:=false ;
    fi;

    path p ;

    if is_minor:
        numeric u ;
        u := .07cm ;
        pair a[] ;

        a[0] := (0,0);
        a[1] := (1.2,0) ;
        a[2] := (1.2,.35) ;
        a[3] := (0,.35) ;

        p := a[0] -- a[1] -- a[2] -- a[3] -- cycle ;
        p := p scaled u  ;
        p := p shifted (.22cm,.15cm) ;
    else:
        p := fullcircle scaled 0 ;
    fi;
    p
enddef;

vardef make_major_seven(expr chord)=
    save is_seven ;
    boolean is_seven ;

    if (length chord>2) and ( substring(1,3) of chord = "M7" ):
        is_seven:=true;
    elseif (length chord>3) and ( substring(2,4) of chord = "M7" ):
        is_seven:=true;
    else:
        is_seven:=false ;
    fi;

    path p[] ;

    if is_seven:
        numeric u ;
        u := 2 ;
        %pickup pencircle scaled 1e-10;
        pair a[] ;

        a[0] := (0,0);
        a[1] := (1,0) ;
        a[2] :=  a[1] rotated 60 ;
        a[3] := (0,0) ;



        p1 := a[0] -- a[1] -- a[2] -- a[3] ;
        transform tt ;
        tt := identity shifted (  - center bbox p1 ) ;
        p2 := p1 transformed tt ;
        p2 := p2 scaled .7 ;
        tt := identity shifted (  center bbox p1 ) ;
        p2 := p2 transformed tt ;

        p1 := p1 scaled u  ;
        p2 := p2 scaled u ;
        p2 := reverse p2 ;
        p2 := p2 -- p1 -- cycle  ;


        p2 := p2 scaled u  shifted (.2cm,.12cm);
    else:
        p2 := fullcircle scaled 0 ;
    fi;
    p2
enddef;

vardef make_seven(expr chord)=
    save is_seven ;
    boolean is_seven ;
    if (length chord>1) and ( substring(1,2) of chord = "7" ):
        is_seven:=true;
    elseif (length chord>2) and ( substring(1,3) of chord = "m7" ):
        is_seven:=true;
    elseif (length chord>2) and ( substring(1,3) of chord = "-7" ):
        is_seven:=true;
    elseif (length chord>3) and ( substring(2,4) of chord = "m7" ):
        is_seven:=true;
    elseif (length chord>3) and ( substring(2,4) of chord = "-7" ):
        is_seven:=true;
    else:
        is_seven:=false ;
    fi;

    path p[] ;

    if is_seven:
        pickup pencircle scaled .1;

        pair a[] ;
        numeric u ;
        u := .5mm ;

        a[0] := (-.2,1)  ;
        a[1] := (.25,.98) ;
        a[2]=(0.5,1) ;
        a[3]=(.4,.5)  ;
        a[4]=(0,0)   ;


        path p[] ;

        p1 := a[0]{1,-1} ... a[1] ... a[2]{dir -65} ... a[3] ... a[4] ;
        p1 := p1 scaled .13 ;
        transform tt ;
        tt := identity shifted (  - center bbox p1 ) ;
        p2 := p1 transformed tt ;
        p2 := p2 scaled .5 ;
        tt := identity shifted (  center bbox p1 ) ;
        p2 := p2 transformed tt ;

        p1 := p1 scaled u ;
        p2 := p2 scaled u ;
        p2 := reverse p2 ;
        p2 := p2 -- p1 -- cycle  ;

        p2 := p2 scaled 1cm ;

        p2 := p2 shifted (.32cm,.03cm) ;


        %p2 := fullcircle scaled 2 ;
    else:
        p2 := fullcircle scaled 0 ;
    fi;
    p2
enddef;


vardef draw_bati(expr p) =
    %save p ;
    %path p ;
    for j=0 upto length p:
            pickup pencircle scaled .05;
            draw (point j of p -- precontrol j of p)   dashed evenly withcolor blue;
            draw (point j of p -- postcontrol j of p)  dashed evenly withcolor blue;
            pickup pencircle scaled .05;
            draw precontrol j of p withcolor red;
            draw postcontrol j of p withcolor red;
            pickup pencircle scaled .05;
            draw point j of p withcolor black;
    endfor;
enddef ;

vardef draw_chord(expr chord,S,background) =
    save q,p ;
    picture q;
    path p;
    interim ahlength := 12bp;
    interim ahangle := 25;
    q := glyph_of_chord (chord) ;
    q := q scaled .01 scaled .8 ;
    transform t ;
    t = identity shifted ( S - center bbox q ) ;
    q := q transformed t ;
    for item within q:
        p := pathpart item ;

        pickup pencircle scaled .001;
        if turningnumber p = 1:
            draw p withcolor black ;
        else:
            draw p withcolor (0,1,0) ;
        fi;
        %draw_bati(p) ;
    endfor ;

    path other ;

    other := make_seven(chord) transformed t ;
    fill other withcolor black ;

    path otherp[] ;
    make_flat(otherp)(chord) ;
    %other := otherp0 transformed t ;
    %draw other withcolor (.5,1,1) ;
    %draw_bati(other) ;
    numeric i ;
    i:=1 ;
    forever:
        if known otherp[i]:
            %draw_bati(otherp[i] transformed t) ;
            i := i+1 ;
        fi;
        exitif unknown otherp[i] ;
    endfor ;

    other := make_sharp(chord) transformed t ;
    pickup pencircle scaled .001;

    other := make_major_seven(chord) transformed t ;
    fill other withcolor black ;

    other := make_minor(chord) transformed t ;
    fill other withcolor black ;

enddef ;


vardef draw_row(expr A,width,height,n,background)(suffix chords) =
    save chord ;
    color c ;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*width,0) ;
    B2 := B1 shifted (0,-height) ;
    B3 := A shifted (0,-height) ;
    draw freehand_path(B0 -- B1 -- B2 -- B3 -- cycle) withcolor c ;

    for i=1 step 1 until n :
        draw freehand_path(B0 shifted (i*width,0) -- B3 shifted (i*width,0)) withcolor c ;
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
    Jingoo.Jg_template.from_string sheet_jingoo
      ~models:
        [
          ("format", Jingoo.Jg_types.Tstr format);
          ("outputtemplate", Jingoo.Jg_types.Tstr outputtemplate);
          ("width", Jingoo.Jg_types.Tstr "1cm");
          ("height", Jingoo.Jg_types.Tstr ".3cm");
          ("section_spacing", Jingoo.Jg_types.Tstr ".5cm");
          ("sections", Jingoo.Jg_types.Tstr sections);
          ("after_sections", Jingoo.Jg_types.Tstr "");
          ("other", Jingoo.Jg_types.Tstr "");
        ]
  in
  let result = clean_string (emit_sheet sheet) in
  fprintf fout "%s" result
