vardef make_sharp(suffix ret)(expr chord)=
	save is_sharp;
	boolean is_sharp ;

	if (length chord>1) and ( substring(1,2) of chord = "#" ):
		is_sharp:=true;
	else:
		is_sharp:=false ;
	fi;

	if is_sharp:
		numeric u ;
		u := .07cm ;

		pair a[] ;

		pair h ; % horizontal
		h := (.5,0) ;
		pair o ; % oblique
		o := h rotated 80 ;
		pair hh,vv ; % small side
		hh := (.2,0) ;
		vv := hh rotated 90 ;
		pair hhh,vvv ; % between bars
		hhh := (.2,0) ;
		vvv := hhh rotated 90 ;

		n:=27 ;

		a0 = (0,0) ;
		a1 = a0 +  h ;
		a2 - a1 = o ;
		a3 = a2 + hh  ;
		a4 = a3 - o ;
		a5 = a4 + hhh ;
		a6 = a5 + o ;
		a7 = a6 + hh ;
		a8 = a7 - o ;
		a9 = a8 + h ;

		a10 = a9 - vv ;
		a16 - a15 = a8-a7 ;

		ypart a11 = ypart a10 = ypart a26 = ypart a27;
		ypart a24 = ypart a25 = ypart a12 = ypart a13;
		ypart a23 = ypart a22 = ypart a19 = ypart a18 = ypart a15 = ypart a14 ;
		ypart a21 = ypart a20 = ypart a17 = ypart a16 ;

		xpart a13 = xpart a14 = xpart a10 ;

		a27-a0 = a23-a24 = -vv ;
		a24 = a27 - vvv ;

		a11 = whatever [a8,a7] ;
		a12 = whatever [a8,a7] ;
		a15 = whatever [a8,a7] ;
		a16 = whatever [a8,a7] ;

		a17 = whatever [a5,a6] ;
		a18 = whatever [a5,a6] ;

		a19 = whatever [a4,a3] ;
		a20 = whatever [a4,a3] ;

		a26 = whatever [a1,a2] ;
		a25 = whatever [a1,a2] ;
		a22 = whatever [a1,a2] ;
		a21 = whatever [a1,a2] ;

		ret0 := a[0] for i=1 step 1 until n: -- a[i] endfor -- cycle ;
		ret0 := ret0 scaled 2 ;
		ret0 := ret0 shifted (6,2) ;

		ret1 :=
	fi;

enddef;
