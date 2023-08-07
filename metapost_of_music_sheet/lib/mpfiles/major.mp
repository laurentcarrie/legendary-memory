

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
