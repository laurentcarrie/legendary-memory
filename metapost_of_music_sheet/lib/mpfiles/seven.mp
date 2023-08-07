vardef make_seven(expr chord)=
    save is_seven ;
    boolean is_seven ;
    if (length chord>1) and ( substring(1,2) of chord = "7" ):
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

    path p[] ;

    if is_seven:
        pickup pencircle scaled .1;

        pair a[] ;
        numeric u ;
        u := .5mm ;

        a[0] := (-.2,1)  ;
        a[1] := (.25,.98) ;
        a[2]=(0.5,1) ;
        a[3]=(.4,.5)  ;
        a[4]=(0,0)   ;


        path p[] ;

        p1 := a[0]{1,-1} ... a[1] ... a[2]{dir -65} ... a[3] ... a[4] ;
        p1 := p1 scaled .13 ;
        transform tt ;
        tt := identity shifted (  - center bbox p1 ) ;
        p2 := p1 transformed tt ;
        p2 := p2 scaled .5 ;
        tt := identity shifted (  center bbox p1 ) ;
        p2 := p2 transformed tt ;

        p1 := p1 scaled u ;
        p2 := p2 scaled u ;
        p2 := reverse p2 ;
        p2 := p2 -- p1 -- cycle  ;

        p2 := p2 scaled 1cm ;

        p2 := p2 shifted (.32cm,.03cm) ;


        %p2 := fullcircle scaled 2 ;
    else:
        p2 := fullcircle scaled 0 ;
    fi;
    p2
enddef;
