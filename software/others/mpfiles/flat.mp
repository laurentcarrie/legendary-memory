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
