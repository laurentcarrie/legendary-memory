prologues:=3;
outputtemplate := "%j-%c.pdf";
outputformat := "pdf";
%outputtemplate := "a-%c.mps";
%outputformat := "mps";

for i:=1 step 1 until 30:
	show(i) ;
	beginfig(i);
		numeric u;
		u := 1cm; % Define the unit length

		numeric a,b,k,ratio ;
		a := .5u ;
		b := 2u ;
		k := (i-1)/29 ;
		ratio := k*b + (1-k) * a ;
		show(ratio) ;
		%    ratio :=1 ;
		% Load the external image
		externalfigure "archi.png" xscaled 3u yscaled 2u ;

		%path aaa ;
        %aaa := fullcircle scaled ratio ;
        %fill aaa withcolor (.5,.2,.2) ;

		% Draw a rectangle around the image
		%draw (-5u, -3u) -- (5u,-3u) -- (5u, 3u) -- (-5u, 3u) -- cycle;

		% Add a label
		%label("My External Image", (2.5u, 1.5u));
	endfig;
endfor;

end.
