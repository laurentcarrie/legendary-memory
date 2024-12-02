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
