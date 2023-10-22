% from
% https://raw.githubusercontent.com/thruston/Drawing-with-Metapost/master/Drawing-with-Metapost.pdf

prologue:=3 ;
outputtemplate := "mps/clock-%c.mps";
outputformat := "mps";

verbatimtex
\documentclass{article}
%%\usepackage{lmodern}
\usepackage[tt=false]{libertine}
\usepackage[libertine]{newtxmath}
\usepackage{amsmath}

\usepackage{luamplib}
\usepackage{graphicx}
\usepackage{fontspec}\setmainfont[Scale=0.6]{Helvetica}
\mplibtextextlabel{enable}
\begin{document}
\begin{mplibcode}

etex

input lib/clock.mp ;

numeric counter ;
counter:=0 ;
for h=1 upto 1:
    for m=0 upto 23:
        counter := counter+1 ;
        beginfig(counter);
            %draw btex \includegraphics[width=5in]{cover.jpg} etex;
            picture p ;
            p := clock(h,m*5) ;
            draw p ;
        endfig ;
    endfor ;
endfor ;

end.
