vardef draw_row(expr A,cell_width,cell_height,n,background)(suffix chords) =
    save chord ;
    color c ;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*cell_width,0) ;
    B2 := B1 shifted (0,-cell_height) ;
    B3 := A shifted (0,-cell_height) ;
    pickup pencircle scaled .05;

    draw freehand_path(B0 -- B1 -- B2 -- B3 -- cycle) withcolor c ;

    for i=1 step 1 until n :
        draw freehand_path(B0 shifted (i*cell_width,0) -- B3 shifted (i*cell_width,0)) withcolor c ;
    endfor ;

    for i=0 step 1 until n-1:
        pair box[] ;
        box0 = B0 shifted (i*cell_width,0) ;
        box1 = box0 shifted (cell_width,0) ;
        box2 = box1 shifted (0,-cell_height) ;
        box3 = box0 shifted (0,-cell_height) ;
        box4 = .5[box0,box2] ;
        pair S ;
        S = .5(box0+box2) ;
        string chord ;
        chord := chords[i] ;
        fill fullcircle scaled 1 shifted S withcolor red ;
        %show(chord) ;
        draw_chord(chord,S,background) ;
    endfor ;

enddef ;
