let make_flat : string =
  {whatever|
  vardef make_flat(suffix ret)(expr chord)=
    save is_flat;
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


		p0 := p0 shifted (.22cm,0) ;
		%ret0 := reverse ret0 ;
		p1 := p1 shifted (.22cm,0) ;
		p1 := reverse p1 ;

		ret0 := p0 ;
		ret1 := p1 ;

	fi;

enddef;
  |whatever}

let make_sharp : string =
  {whatever|
  vardef make_sharp(suffix ret)(expr chord)=
	save is_sharp;
	boolean is_sharp ;

	if (length chord>1) and ( substring(1,2) of chord = "#" ):
		is_sharp:=true;
	else:
		is_sharp:=false ;
	fi;

	if is_sharp:
		numeric u ;
		u := .07cm ;

		pair a[] ;

		pair h ; % horizontal
		h := (.5,0) ;
		pair o ; % oblique
		o := h rotated 80 ;
		pair hh,vv ; % small side
		hh := (.2,0) ;
		vv := hh rotated 90 ;
		pair hhh,vvv ; % between bars
		hhh := (.2,0) ;
		vvv := hhh rotated 90 ;

		n:=27 ;

		a0 = (0,0) ;
		a1 = a0 +  h ;
		a2 - a1 = o ;
		a3 = a2 + hh  ;
		a4 = a3 - o ;
		a5 = a4 + hhh ;
		a6 = a5 + o ;
		a7 = a6 + hh ;
		a8 = a7 - o ;
		a9 = a8 + h ;

		a10 = a9 - vv ;
		a16 - a15 = a8-a7 ;

		ypart a11 = ypart a10 = ypart a26 = ypart a27;
		ypart a24 = ypart a25 = ypart a12 = ypart a13;
		ypart a23 = ypart a22 = ypart a19 = ypart a18 = ypart a15 = ypart a14 ;
		ypart a21 = ypart a20 = ypart a17 = ypart a16 ;

		xpart a13 = xpart a14 = xpart a10 ;

		a27-a0 = a23-a24 = -vv ;
		a24 = a27 - vvv ;

		a11 = whatever [a8,a7] ;
		a12 = whatever [a8,a7] ;
		a15 = whatever [a8,a7] ;
		a16 = whatever [a8,a7] ;

		a17 = whatever [a5,a6] ;
		a18 = whatever [a5,a6] ;

		a19 = whatever [a4,a3] ;
		a20 = whatever [a4,a3] ;

		a26 = whatever [a1,a2] ;
		a25 = whatever [a1,a2] ;
		a22 = whatever [a1,a2] ;
		a21 = whatever [a1,a2] ;

		ret0 := a[0] for i=1 step 1 until n: -- a[i] endfor -- cycle ;
		ret0 := ret0 scaled 2 ;
		ret0 := ret0 shifted (6,2) ;

		ret1 :=
	fi;

enddef;
  |whatever}

let make_seven : string =
  {whatever|
  vardef make_seven(suffix ret)(expr chord)=
    boolean is_seven ;
    if (length chord>1) and ( substring(1,2) of chord = "7" ):
        is_seven:=true;
    elseif (length chord>2) and ( substring(2,3) of chord = "7" ):
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
        %ret0 := reverse ret0 ;


	fi;

enddef;
  |whatever}

let make_minor : string =
  {whatever|
  vardef make_minor(expr chord)=
    save is_minor;
    boolean is_minor ;

    if (length chord>1) and ( substring(1,2) of chord = "m" ):
        is_minor:=true;
    elseif (length chord>1) and ( substring(1,2) of chord = "-" ):
        is_minor:=true ;
    elseif (length chord>2) and ( substring(2,3) of chord = "m" ):
        is_minor:=true;
    elseif (length chord>2) and ( substring(2,3) of chord = "-" ):
        is_minor:=true ;
    else:
        is_minor:=false ;
    fi;

    path p ;

    if is_minor:
        numeric u ;
        u := .07cm ;
        pair a[] ;

        a[0] := (0,0);
        a[1] := (1.2,0) ;
        a[2] := (1.2,.35) ;
        a[3] := (0,.35) ;

        p := a[0] -- a[1] -- a[2] -- a[3] -- cycle ;
        p := p scaled u  ;
        p := p shifted (.22cm,.15cm) ;
    else:
        p := fullcircle scaled 0 ;
    fi;
    p
enddef;
  |whatever}

let make_major_seven : string =
  {whatever|
  vardef make_major_seven(expr chord)=
    save is_seven ;
    boolean is_seven ;

    if (length chord>2) and ( substring(1,3) of chord = "M7" ):
        is_seven:=true;
    elseif (length chord>3) and ( substring(2,4) of chord = "M7" ):
        is_seven:=true;
    else:
        is_seven:=false ;
    fi;

    path p[] ;

    if is_seven:
        numeric u ;
        u := 2 ;
        %pickup pencircle scaled 1e-10;
        pair a[] ;

        a[0] := (0,0);
        a[1] := (1,0) ;
        a[2] :=  a[1] rotated 60 ;
        a[3] := (0,0) ;



        p1 := a[0] -- a[1] -- a[2] -- a[3] ;
        transform tt ;
        tt := identity shifted (  - center bbox p1 ) ;
        p2 := p1 transformed tt ;
        p2 := p2 scaled .7 ;
        tt := identity shifted (  center bbox p1 ) ;
        p2 := p2 transformed tt ;

        p1 := p1 scaled u  ;
        p2 := p2 scaled u ;
        p2 := reverse p2 ;
        p2 := p2 -- p1 -- cycle  ;


        p2 := p2 scaled u  shifted (.2cm,.12cm);
    else:
        p2 := fullcircle scaled 0 ;
    fi;
    p2
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
  vardef draw_row(expr A,width,height,n,background)(suffix chords) =
    save chord ;
    color c ;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*width,0) ;
    B2 := B1 shifted (0,-height) ;
    B3 := A shifted (0,-height) ;
    draw freehand_path(B0 -- B1 -- B2 -- B3 -- cycle) withcolor c ;

    for i=1 step 1 until n :
        draw freehand_path(B0 shifted (i*width,0) -- B3 shifted (i*width,0)) withcolor c ;
    endfor ;

    for i=0 step 1 until n-1:
        pair box[] ;
        box0 = B0 shifted (i*width,0) ;
        box1 = box0 shifted (width,0) ;
        box2 = box1 shifted (0,-height) ;
        box3 = box0 shifted (0,-height) ;
        box4 = .5[box0,box2] ;
        pair S ;
        S = .5(box0+box2) ;
        string chord ;
        chord := chords[i] ;
        %show(chord) ;
        draw_chord(chord,S,background) ;
    endfor ;

enddef ;
  |whatever}

let make_draw_chord : string =
  {whatever|
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
  |whatever}
