open Printf

let draw_row_jingoo : string = {whatever|

|whatever}
;;


let mp_jingoo : string = {whatever|
prologues:=3;
outputtemplate := "mps/frame_%c.{{format}}";
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

def drawrow(expr A,n,width,height) =
    color c ;
    c := (0,0,0) ;
    pair B,C,D ;
    B := A shifted (n*width,0) ;
    C := B shifted (0,height) ;
    D := A shifted (0,height) ;
    draw A -- B -- C -- D -- cycle withcolor c ;
    %draw origin -- (10cm,10cm) ;
    %numeric i;
    for i=1 step 1 until n-1 :
        draw A shifted (i*width,0) -- D shifted (i*width,0) withcolor c ;
    endfor ;

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
    drawrow(A,4,width,height) ;


enddef ;


beginfig(0);
    mygrida (0) ;
endfig;
end.
|whatever}
;;


let emit fout sheet format =
  let _ = fprintf fout "%%%s \n" sheet.Sheet.title in
(*  let extension = match format with *)
(*  | "png" -> "png" *)
(*  |"mp" -> "mp" *)
(*  |_-> failwith "bad format" *)
(*  in *)

(*    let range from until = *)
(*        List.init (until - from) (fun i -> Jingoo.Jg_types.Tint (i + from)) *)
(*    in *)

    let result :string = Jingoo.Jg_template.from_string mp_jingoo
    ~models:[
    ("format", Jingoo.Jg_types.Tstr format);
    ("width", Jingoo.Jg_types.Tstr "1cm" );
    ("height", Jingoo.Jg_types.Tstr ".5cm" );
    ("n",Jingoo.Jg_types.Tint 7)
    ]

  in
  let _ = fprintf fout "%s\n" result in
  ()
