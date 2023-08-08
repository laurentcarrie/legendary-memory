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
