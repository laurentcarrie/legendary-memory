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

    %other := make_seven(chord) transformed t ;
    %fill other withcolor black ;

    path otherp[] ;
    make_flat(otherp)(chord) ;
    other := otherp0 transformed t ;
    draw other withcolor (.5,1,1) ;
    draw_bati(other) ;
    numeric i ;
    i:=1 ;
    forever:
        if known otherp[i]:
            %draw_bati(otherp[i] transformed t) ;
            i := i+1 ;
        fi;
        exitif unknown otherp[i] ;
    endfor ;

    %other := make_sharp(chord) transformed t ;
    %pickup pencircle scaled .001;

    %other := make_major_seven(chord) transformed t ;
    %fill other withcolor black ;

    %other := make_minor(chord) transformed t ;
    %fill other withcolor black ;

enddef ;
