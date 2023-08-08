vardef draw_one(suffix p)(expr t,background) =
    numeric i,tn ;
    i:=0 ;
    pickup pencircle scaled .05;
    forever:
        if known p[i]:
            pickup pencircle scaled .05;
            show "turning number" ;
            show turningnumber p[i] ;
            tn := turningnumber p[i] ;
            %tn := tn - 2 * ( tn div 2 ) ;
            show tn ;
            if tn = 1 :
                fill p[i] transformed t withcolor green ;
            elseif tn = -1:
                fill p[i] transformed t withcolor blue ;
            else:
                fill p[i] transformed t withcolor red ;
            fi;
            draw_bati(p[i] transformed t) ;
            i := i+1 ;
        fi;
        exitif unknown p[i] ;
    endfor ;
enddef ;


vardef draw_chord(expr chord,S,background) =
    boolean do_draw_bati ;
    do_draw_bati := false ;

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
            fill p withcolor black ;
        else:
            fill p withcolor background ;
        fi;
        if do_draw_bati :
            draw_bati(p) ;
        fi ;
    endfor ;

    path other ;

    path otherp[] ;
    make_flat(otherp)(chord) ;
    draw_one(otherp)(t,background) ;

    make_sharp(otherp)(chord) ;
    draw_one(otherp)(t,background) ;

    make_seven(otherp)(chord) ;
    draw_one(otherp)(t,background) ;


    %other := make_major_seven(chord) transformed t ;
    %fill other withcolor black ;

    %other := make_minor(chord) transformed t ;
    %fill other withcolor black ;

enddef ;
