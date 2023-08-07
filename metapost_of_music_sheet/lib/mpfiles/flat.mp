vardef make_flat(suffix ret)(expr chord)=
    save is_flat;
	boolean is_flat ;
	if (length chord>1) and ( substring(1,2) of chord = "b" ): is_flat:=true;
	else: is_flat:=false ;
	fi;
	path p ;
	if is_flat: numeric u,n ;
		u := 2 ;
		pair a[] ;
		n:=3 ;
		a0 := (0,0) ;
		a1 := a0 shifted (0,4.5) ;
		a2 := a1 shifted (.18,0) ;
		a3 := a2 shifted (0,-2.43) ;
		a4 - a3 = dir (30) scaled .8 ;
		a5 - a4 = dir (-30) scaled .9 ;
		a6 - a5 = dir (-90) scaled .8 ;
		p := a0 -- a1 -- a2 -- a3 -- a4 {dir 20} .. a5  .. a6 .. cycle  ;
		p := p shifted (.22cm,0) ;
	else:
	    p := fullcircle scaled 0 ;
	fi;
	ret[0] := p ;
enddef;
