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
