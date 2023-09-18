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
		v := 1 ;
		pair a[] ;
		a0 := (0,0) ;
		a1 := (u,0) ;
		a2 := (u,v) ;
		a3 := (0,v) ;
		p0 := a0 -- a1 -- a2 -- a3 -- cycle  ;

		pair b[];
        b0 := (0,0) ;
        b1 := (1,0) ;
        b2 := (u/2,2) ;
        b3-b0 = b2-b1 ;
		p1 := b0 -- b1 -- b2 -- b3 -- cycle  ;

		p2 := a1 -- b1 -- b2 -- b3 -- cycle  ;


        transform t ;
		t := identity shifted (5,0) ;

		p0 := p0 transformed t ;
		p1 := p1 transformed t ;

		ret0 := p0 scaled chord_glyph_scale ;
		ret1 := p1 scaled chord_glyph_scale ;


	fi;

enddef;
