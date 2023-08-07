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
