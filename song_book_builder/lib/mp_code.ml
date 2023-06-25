let make_flat : string =
  {whatever|
  vardef make_flat(suffix ret)(expr chord)=
    save is_flat,t;
	boolean is_flat ;
	if (length chord>1) and ( substring(1,2) of chord = "b" ): is_flat:=true;
	else: is_flat:=false ;
	fi;
	path p[] ;
	if is_flat:
	    numeric u,n ;
		u := 2 ;
		pair a[] ;
		n:=3 ;
		a0 := (0,0) ;
		a1 := a0 shifted (0,4.5) ;
		a2 := a1 shifted (.28,0) ;
		a3 := a2 shifted (0,-2.43) ;
		a4 - a3 = dir (30) scaled .8 ;
		a5 - a4 = dir (-30) scaled .9 ;
		a6 - a5 = dir (-90) scaled .8 ;
		p0 := a0 -- a1 -- a2 -- a3 -- a4 {dir 20} .. a5  .. a6 .. cycle  ;

		pair b[] ;
		b0 := a3 shifted (0,-.2) ;
		b1 - b0 = dir(30) scaled .6 ;
		b2 - b1 = dir(-30) scaled .6 ;
		b3 - b2 = dir(-90) scaled .6 ;
		b4 - b0 = whatever * dir(90) ;
		b4 - a0 = whatever * dir(45) ;
		p1 := b0 -- b1 {dir 20} .. b2 .. b3 .. b4 -- cycle ;

        transform t ;
		t := identity shifted (0.18cm,-.2cm) ;
		%t := identity ;

		p0 := p0 transformed t ;
		p0 := reverse p0 ;
		p1 := p1 transformed t ;
		%p1 := reverse p1 ;

		ret0 := p0 scaled chord_glyph_scale ;
		ret1 := p1 scaled chord_glyph_scale ;

	fi;

enddef;
  |whatever}

let make_sharp : string =
  {whatever|
  vardef make_sharp(suffix ret)(expr chord)=
    save is_sharp,t;
	boolean is_sharp ;
	if (length chord>1) and ( substring(1,2) of chord = "#" ): is_sharp:=true;
	else: is_sharp:=false ;
	fi;
	path p[] ;
	if is_sharp:
	    numeric u,n ;
		u := 3.2 ;
		v := .5 ;
		pair a[] ;
		n:=3 ;
		a0 := (0,0) ;
		a1 := (u,0) ;
		a2 := (u,v) ;
		a3 := (0,v) ;
		p0 := a0 -- a1 -- a2 -- a3 -- cycle  ;
		p1 := p0 shifted (0,-1.2) ;

		pair b[] ;
		b0 = (0.5,-3) ;
		b1 = b0 shifted (.5,0) ;
		b2 = b1 shifted (1,5) ;
		b3-b0 = b2-b1 ;
		p2 = b0 -- b1 -- b2 -- b3 -- cycle ;
		p3 = p2 shifted (1.0,0) ;

        transform t ;

		t := identity scaled .8 shifted (4.0,-3) ;

		p0 := p0 transformed t ;
		p1 := p1 transformed t ;
		p2 := p2 transformed t ;
		p3 := p3 transformed t ;

		ret0 := p0 scaled chord_glyph_scale ;
		ret1 := p1 scaled chord_glyph_scale ;
		ret2 := p2 scaled chord_glyph_scale ;
		ret3 := p3 scaled chord_glyph_scale ;

	fi;

enddef;
  |whatever}

let make_seven : string =
  {whatever|
  vardef make_seven(suffix ret)(expr chord)=
    boolean is_seven ;
    if (length chord>1) and ( substring(1,2) of chord = "7" ):
        is_seven:=true;
    elseif (length chord>2) and (substring(1,2) of chord = "b") and ( substring(2,3) of chord = "7" ):
        is_seven:=true;
    elseif (length chord>2) and (substring(1,2) of chord = "#") and ( substring(2,3) of chord = "7" ):
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
	if is_seven:
	    numeric u,n ;
		u := 2 ;
		pair a[] ;
		n:=3 ;
		a0 := (6.6,2.9) ;
		a1 - a0 = dir(70) scaled 2 ;
		a2 - a1 = dir(180) scaled 1 ;
		a3 - a2 = dir(90) scaled .4 ;

		a4 = .5(a3+a5) shifted (0,-.05) ;

		a5 - a3 = dir(0) scaled 1.7 ;
		.8 * xpart (a5 - a1) =  xpart (a6 - a0) ;
		a6 - a0 = whatever * dir(0) ;
		ret0 := a0 -- a1 -- a2{dir 180} .. a3{dir 0} .. a4 .. {dir 0}a5 -- a6 {dir(-110)} .. cycle ;
        ret0 := reverse ret0 ;

        ret0 := ret0 scaled 1.2 shifted (-3,-1.5) ;

        ret0 := ret0 scaled chord_glyph_scale ;


	fi;

enddef;
  |whatever}

let make_minor : string =
  {whatever|
  vardef make_minor(suffix ret)(expr chord)=
    save is_minor,t;
	boolean is_minor ;
	if (length chord>1) and ( substring(1,2) of chord = "m" ): is_minor:=true;
	elseif (length chord>2) and ( substring(1,2) of chord = "b" ) and ( substring(2,3) of chord = "m" ) : is_minor:=true;
	elseif (length chord>2) and ( substring(1,2) of chord = "#" ) and ( substring(2,3) of chord = "m" ) : is_minor:=true;
	else: is_minor:=false ;
	fi;
	path p[] ;
	if is_minor:
	    numeric u,n ;
		u := 2 ;
		v := 1 ;
		pair a[] ;
		n:=3 ;
		a0 := (0,0) ;
		a1 := (u,0) ;
		a2 := (u,v) ;
		a3 := (0,v) ;
		p0 := a0 -- a1 -- a2 -- a3 -- cycle  ;

        transform t ;
		t := identity shifted (5,0) ;

		p0 := p0 transformed t ;

		ret0 := p0 scaled chord_glyph_scale ;

	fi;

enddef;
  |whatever}

let make_major_seven : string =
  {whatever|
  vardef make_major_seven(suffix ret)(expr chord)=
    save is_major_seven,t;
	boolean is_major_seven ;
	if (length chord>2) and ( substring(1,3) of chord = "M7" ): is_major_seven:=true;
	elseif (length chord>3) and ( substring(1,2) of chord = "b" ) and ( substring(2,4) of chord = "M7" ) : is_major_seven:=true;
	elseif (length chord>3) and ( substring(1,2) of chord = "#" ) and ( substring(2,4) of chord = "M7" ) : is_major_seven:=true;
	else: is_major_seven:=false ;
	fi;


	path p[] ;
	if is_major_seven:
	    numeric u,n ;
		u := 2 ;
		pair a[] ;
		path p[] ;
		a0 := dir(90) scaled 1 ;
		a1 := dir(210) scaled 1 ;
		a2 := dir(330) scaled 1 ;
		p0 := a0 -- a1 -- a2 -- cycle ;
        %p0 := reverse p0 ;


        p0 := p0 scaled 2.2 ;
        p1 := p0 scaled 0.8 ;
        p1 := reverse p1 ;


        p0 := p0 shifted (5.4,2) ;
        p1 := p1 shifted (5.4,2) ;

        ret0 := p0 scaled chord_glyph_scale ;
        ret1 := p1 scaled chord_glyph_scale ;

	fi;

enddef;
  |whatever}

let make_draw_bati : string =
  {whatever|
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
            pickup pencircle scaled .15;
            draw point j of p withcolor green;
    endfor;
enddef ;
  |whatever}

let make_glyph_of_chord : string =
  {whatever|
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
  |whatever}

let make_draw_row : string =
  {whatever|
  vardef draw_row(expr A,cell_width,cell_height,n,background)(suffix chords,barindex,nbchordsinbar,indexinbar) =
    save chord ;
    color c ;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*cell_width,0) ;
    B2 := B1 shifted (0,-cell_height) ;
    B3 := A shifted (0,-cell_height) ;
    pickup pencircle scaled .05;

    draw freehand_path(B0 -- B1 -- B2 -- B3 -- cycle) withcolor c ;

    for i=1 step 1 until n :
        draw freehand_path(B0 shifted (i*cell_width,0) -- B3 shifted (i*cell_width,0)) withcolor c ;
    endfor ;

    for i=0 step 1 until n-1:
        pair box[] ;
        bar := barindex[i] ;
        nbchordsinbar := nbchordsinbar[i] ;
        indexinbar := indexinbar[i] ;
        box0 = B0 shifted (bar*cell_width,0) ;
        box1 = box0 shifted (cell_width,0) ;
        box2 = box1 shifted (0,-cell_height) ;
        box3 = box0 shifted (0,-cell_height) ;
        box4 = .5[box0,box2] ;
        pair S ;
        numeric subboxwidth ;
        show("indexinbar") ;
        show(indexinbar) ;
        show("nbchordsinbar") ;
        show(nbchordsinbar) ;
        subboxwidth = cell_width / nbchordsinbar ;
        S = .5[box0 shifted (indexinbar*subboxwidth,0),box3 shifted ((indexinbar+1)*subboxwidth,0)] ;
        string chord ;
        chord := chords[i] ;
        %fill fullcircle scaled 1 shifted S withcolor red ;
        %show(chord) ;
        draw_chord(chord,S,background) ;
    endfor ;

enddef ;
  |whatever}

let make_draw_chord : string =
  {whatever|
  numeric ratio ;
ratio = .5 * .1 ;

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
            if (tn = 1) or (tn = 0) :
                fill p[i] transformed t withcolor black ;
            elseif tn = -1:
                fill p[i] transformed t withcolor background ;
            else:
                fill p[i] transformed t withcolor red ;
            fi;
            %draw_bati(p[i] transformed t) ;
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
    q := q scaled (chord_glyph_scale *.01) ;
    transform t,tt ;
    t = identity shifted ( S - center bbox q ) ;
    tt = identity shifted S ;
    % t = identity scaled ratio shifted ( S - center bbox q ) ;
    q := q transformed t  ;
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
    draw_one(otherp)(tt,background) ;

    make_sharp(otherp)(chord) ;
    draw_one(otherp)(tt,background) ;

    make_seven(otherp)(chord) ;
    draw_one(otherp)(tt,background) ;

    make_major_seven(otherp)(chord) ;
    draw_one(otherp)(tt,background) ;

    make_minor(otherp)(chord) ;
    draw_one(otherp)(tt,background) ;



enddef ;
  |whatever}
