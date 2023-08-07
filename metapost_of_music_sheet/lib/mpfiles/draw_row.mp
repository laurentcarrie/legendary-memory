vardef draw_row(expr A,width,height,n,background)(suffix chords) =
    save chord ;
    color c ;
    c := (0,0,0) ;
    B0 := A ;
    B1 := A shifted (n*width,0) ;
    B2 := B1 shifted (0,-height) ;
    B3 := A shifted (0,-height) ;
    draw freehand_path(B0 -- B1 -- B2 -- B3 -- cycle) withcolor c ;

    for i=1 step 1 until n :
        draw freehand_path(B0 shifted (i*width,0) -- B3 shifted (i*width,0)) withcolor c ;
    endfor ;

    for i=0 step 1 until n-1:
        pair box[] ;
        box0 = B0 shifted (i*width,0) ;
        box1 = box0 shifted (width,0) ;
        box2 = box1 shifted (0,-height) ;
        box3 = box0 shifted (0,-height) ;
        box4 = .5[box0,box2] ;
        pair S ;
        S = .5(box0+box2) ;
        string chord ;
        chord := chords[i] ;
        %show(chord) ;
        draw_chord(chord,S,background) ;
    endfor ;

enddef ;
