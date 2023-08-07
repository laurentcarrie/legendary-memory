let make_flat : string =
  {whatever|
  vardef make_flat(suffix ret)(expr chord)= save is_flat;
 boolean is_flat ;
 if (length chord>1) and ( substring(1,2) of chord = "b" ): is_flat:=true;
 else: is_flat:=false ;
 fi;
 path p ;
 if is_flat: numeric u,n ;
 u := 2 ;
 pair a[] ;
 n:=3 ;
 a[0] := (2.8,0.4);
 a[1] := (3,0.4) ;
 a[2] := (3,2.6) ;
 a[3] := (2.8,2.6) ;
 a[4] := (3,.4) ;
 a[5] := (3,2.6) ;
 p := a[0] for i=1 step 1 until n: -- a[i] endfor ;
 path q ;
 q := a[4] -- a[5] ;
 p := p -- q -- cycle ;
 p := p scaled u ;
 else: p := fullcircle scaled 0 ;
 fi;
 ret[0] := p ;
 enddef;
  |whatever}

let make_sharp : string =
  {whatever|
  vardef make_sharp(expr chord)=
    save is_sharp;
    boolean is_sharp ;

    if (length chord>1) and ( substring(1,2) of chord = "#" ):
        is_sharp:=true;
    else:
        is_sharp:=false ;
    fi;
    save p ;
    path p ;

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


        path p ;
        p := a[0] for i=1 step 1 until n: -- a[i] endfor -- cycle ;
        p := p scaled 2 ;
        p := p shifted (6,2) ;

    else:
        p := fullcircle scaled 0 ;
    fi;
    p
enddef;
  |whatever}
