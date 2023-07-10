open Printf

let draw_row_jingoo : string = {whatever|

|whatever}

let mp_jingoo : string =
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
    if chord="A": p:=glyph "A" of "ec-lmr10" ; fi;
    if chord="B": p:=glyph "B" of "ec-lmr10" ; fi;
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
        show("XXXXXXXXXXXXXXXXXXXXXXXX");
        show(turningnumber p) ;
        %draw p withcolor (.6,.9,.6) withpen pencircle scaled .5;

        %drawarrow p withcolor (.6,.9,.6) withpen pencircle scaled .5;
        %draw p withcolor (.6,.9,.6) withpen pencircle scaled .5;
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


vardef drawrow(suffix B)(expr A,width,height,n,background)(suffix chords) =
    color c ;
    show(c) ;
    show(A) ;
    show(width);
    show(height);
    show(chords[0]) ;
    %numeric n ;
    %n := length chords ;
    %draw chords0 withcolor (0,1,0) ;
    %n:=2;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*width,0) ;
    B2 := B1 shifted (0,-height) ;
    B3 := A shifted (0,-height) ;
    draw B0 -- B1 -- B2 -- B3 -- cycle withcolor c ;

    for i=1 step 1 until n :
        draw B0 shifted (i*width,0) -- B3 shifted (i*width,0) withcolor c ;
    endfor ;

    string p ;
    for i=0 step 1 until n-1:
        pair box[] ;
        box0 = B0 shifted (i*width,0) ;
        box1 = box0 shifted (width,0) ;
        box2 = box1 shifted (0,-height) ;
        box3 = box0 shifted (0,-height) ;
        box4 = .5[box0,box2] ;
        p := chords[i] ;
        pair S ;
        S = .5(box0+box2) ;
        show("line 117");
        show(S) ;
        show("line 119");
        string chord ;
        show(chords) ;
        chord := "A";
        draw_chord(chord,S,background) ;
        %draw p shifted box4 withcolor (1,0,0) ;
        %dotlabel.urt("xx",box4) ;
    endfor ;

    %draw chords0 ;


enddef ;


vardef bchord =
    path p ;
    p := (-3,-3) -- (3,0) -- (0,0);
    p scaled 1cm
enddef ;



def mygrida(expr t)=
    u:=.2cm ;
    margin:=4cm ;
    path p ;
    p := (-margin,-margin) -- (-margin,margin) -- (margin,margin) --
    (margin,-margin)  -- cycle ;
    color background ;
    background := (.8,.7,.7) ;
    fill p withcolor background ;
    label(decimal t,(-margin,-margin)/2) ;
    %%draw textext("cycle " & decimal t) shifted (-margin,-margin)/2  ;

    numeric n,width,height ;
    n := {{n}} ;
    pair A ;
    width := {{width}} ;
    height := {{height}} ;
    A = (-3cm,3cm) ;

    string chords[] ;
    chords0 := "a"  ;
    chords1 := "b" ;
    show(chords) ;
    pair B[] ;
    drawrow(B)(A,width,height,2,background,chords) ;
    %draw chords0 withcolor (0,1,0) ;


enddef ;


beginfig(0);
    mygrida (0) ;
endfig;
end.
|whatever}

let emit fout sheet format outputtemplate =
  let _ = fprintf fout "%%%s \n" sheet.Sheet.title in

  (*  let extension = match format with *)
  (*  | "png" -> "png" *)
  (*  |"mp" -> "mp" *)
  (*  |_-> failwith "bad format" *)
  (*  in *)

  (*    let range from until = *)
  (*        List.init (until - from) (fun i -> Jingoo.Jg_types.Tint (i + from)) *)
  (*    in *)
  let result : string =
    Jingoo.Jg_template.from_string mp_jingoo
      ~models:
        [
          ("format", Jingoo.Jg_types.Tstr format);
          ("outputtemplate", Jingoo.Jg_types.Tstr outputtemplate);
          ("width", Jingoo.Jg_types.Tstr "1cm");
          ("height", Jingoo.Jg_types.Tstr ".5cm");
          ("n", Jingoo.Jg_types.Tint 7);
        ]
  in

  let _ = fprintf fout "%s\n" result in
  ()
