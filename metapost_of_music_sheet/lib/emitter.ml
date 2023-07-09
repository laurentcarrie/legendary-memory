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

vardef drawrow(suffix B)(expr A,width,height,n)(suffix chords) =
    color c ;
    show(c) ;
    show(A) ;
    show(width);
    show(height);
    show(chords[0]) ;
    %numeric n ;
    %n := length(chords) ;
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

    path p ;
    for i=0 step 1 until n-1:
        pair box[] ;
        box0 = B0 shifted (i*width,0) ;
        box1 = box0 shifted (width,0) ;
        box2 = box1 shifted (0,-height) ;
        box3 = box0 shifted (0,-height) ;
        box4 = .5[box0,box2] ;
        p := chords[i] ;
        %draw p shifted box4 withcolor (1,0,0) ;
        %dotlabel.urt("xx",box4) ;
    endfor ;



    {% for chord in chords %}
    {{ chord }}
    {% endfor %}

    %draw chords0 ;


enddef ;

vardef achord =
    path p[] ;
    p0 := (-1,-1) -- (0,1) ;
    p
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
    fill p withcolor (.8,.7,.7) ;
    label(decimal t,(-margin,-margin)/2) ;
    %%draw textext("cycle " & decimal t) shifted (-margin,-margin)/2  ;

    numeric n,width,height ;
    n := {{n}} ;
    pair A ;
    width := {{width}} ;
    height := {{height}} ;
    A = (-3cm,3cm) ;

    path chords[] ;
    chords0 := achord  ;
    chords1 := achord ;
    show(chords) ;
    pair B[] ;
    drawrow(B)(A,width,height,4,chords) ;
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
