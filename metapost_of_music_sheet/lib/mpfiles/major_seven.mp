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
		pair a[] ;
		path p[] ;
		a0 := (0,0) ;
		a1 - a0 = dir(0) scaled 1 ;
		a2 - a0 = dir(60) scaled 1 ;
		p0 := a0 -- a1 -- a2 -- cycle ;
        %p0 := reverse p0 ;


        p0 := p0 scaled 2.8 ;
        p1 := p0 scaled 1.2 ;


        p0 := p0 shifted (4.2,0) ;
        p1 := p1 shifted (4.2,0) ;

        ret0 := p1 scaled chord_glyph_scale ;
        %ret1 := p0 scaled chord_glyph_scale ;

	fi;

enddef;
