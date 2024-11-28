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
