open Printf

let other =
  {whatever|

beginfig(1) ;

pair O ;
O := (0,0) ;
draw_chord("Ab7",O,background) ;

path pp ;
pickup pencircle scaled .01 ;
pp := (-5,0) -- (5,0) ;
for i:=-3 upto 3 :
    draw pp shifted (0,i) withcolor blue ;
    %dotlabel.top(decimal i,(-5,i)) scaled .1 withcolor blue ;
endfor ;

pp := (0,-5) -- (0,5) ;
for i:=-3 upto 3 :
    draw pp shifted (i,0) withcolor blue ;
endfor ;

endfig ;

beginfig(3) ;

z1=(5mm,5mm) ;
drawarrow origin -- z1 ;
label("hello world",z1) withcolor blue ;
label.urt("hello world",z1) withcolor red ;
%draw thelabel.rt("Hello" & " " & "San Diego!",origin)
%    xscaled 0.7
%    rotated 60 withcolor green ;
endfig ;

beginfig(2) ;
z1 = right*28mm ;
z2 = right*30mm ;
z3 = right*33mm ;
draw origin ;
for i=0 step 10 until 350:
    label (decimal i ,z3 rotated i) ;
    draw (z1--z2) rotated i ;
endfor ;
endfig ;



|whatever}

let main () =
  let result =
    Jingoo.Jg_template.from_string Metapost_of_music_sheet.Emitter.sheet_jingoo
      ~models:
        [
          ("cell_width", Jingoo.Jg_types.Tint 200);
          ("cell_height", Jingoo.Jg_types.Tint 200);
          ("section_spacing", Jingoo.Jg_types.Tint 2);
          ("outputtemplate", Jingoo.Jg_types.Tstr "mps/chord-%c.mps");
          ("outputformat", Jingoo.Jg_types.Tstr "mps");
          ("after_sections", Jingoo.Jg_types.Tstr "");
          ("other", Jingoo.Jg_types.Tstr other);
          ( "vardef_make_flat",
            Jingoo.Jg_types.Tstr Metapost_of_music_sheet.Mp_code.make_flat );
          ( "vardef_make_sharp",
            Jingoo.Jg_types.Tstr Metapost_of_music_sheet.Mp_code.make_sharp );
          ( "vardef_make_draw_chord",
            Jingoo.Jg_types.Tstr Metapost_of_music_sheet.Mp_code.make_draw_chord
          );
          ( "vardef_make_glyph_of_chord",
            Jingoo.Jg_types.Tstr
              Metapost_of_music_sheet.Mp_code.make_glyph_of_chord );
          ( "vardef_make_seven",
            Jingoo.Jg_types.Tstr Metapost_of_music_sheet.Mp_code.make_seven );
          ( "vardef_make_major_seven",
            Jingoo.Jg_types.Tstr
              Metapost_of_music_sheet.Mp_code.make_major_seven );
          ( "vardef_make_minor",
            Jingoo.Jg_types.Tstr Metapost_of_music_sheet.Mp_code.make_minor );
          ( "vardef_make_draw_bati",
            Jingoo.Jg_types.Tstr Metapost_of_music_sheet.Mp_code.make_draw_bati
          );
        ]
  in
  let result = Metapost_of_music_sheet.Emitter.clean_string result in
  let fout = open_out "chord.mp" in
  let _ = fprintf fout "%s\n" result in
  let _ = close_out fout in
  (*  let _ = Metapost_of_music_sheet.Pdf.mps_of_mp sheet in *)
  (*  let _ = Metapost_of_music_sheet.Pdf.pdf_of_tex "chord" in *)
  ()
;;

let _ = main () in
()
