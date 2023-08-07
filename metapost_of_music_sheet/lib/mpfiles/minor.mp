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
