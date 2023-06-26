open Printf

let emit fout sheet =
  let _ = fprintf fout "%%%s \n" sheet.Sheet.title in
    let _ = fprintf fout "
prologues:=3;
outputtemplate := \"mps/frame_%%c.mps\";
outputformat := \"mps\";

input boxes ;
input TEX ;

verbatimtex
\\documentclass{article}
%%\\usepackage{lmodern}
  \\usepackage[tt=false]{libertine}
  \\usepackage[libertine]{newtxmath}
\\usepackage{amsmath}
\\begin{document}
etex

def mygrida(expr t)=
	u:=.2cm ;
	margin:=4cm ;
	draw (-margin,-margin) -- (-margin,margin) -- (margin,margin) -- (margin,-margin)  withcolor white	;
	 label(decimal t,(-margin,-margin)/2) ;
	%%draw textext(\"cycle \" & decimal t) shifted (-margin,-margin)/2  ;
enddef ;


beginfig(0);
	    mygrida (0) ;
endfig;

end.

"  in
()