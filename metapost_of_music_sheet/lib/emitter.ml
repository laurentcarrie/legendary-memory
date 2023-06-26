open Printf

let emit fout sheet =
  let _ = fprintf fout "%%%s \n" sheet.Sheet.title in
  let _ =
    fprintf fout
      "\n\
       prologues:=3;\n\
       outputtemplate := \"mps/frame_%%c.mps\";\n\
       outputformat := \"mps\";\n\n\
       input boxes ;\n\
       input TEX ;\n\n\
       verbatimtex\n\
       \\documentclass{article}\n\
       %%\\usepackage{lmodern}\n\
      \  \\usepackage[tt=false]{libertine}\n\
      \  \\usepackage[libertine]{newtxmath}\n\
       \\usepackage{amsmath}\n\
       \\begin{document}\n\
       etex\n\n\
       def mygrida(expr t)=\n\
       \tu:=.2cm ;\n\
       \tmargin:=4cm ;\n\
       \tdraw (-margin,-margin) -- (-margin,margin) -- (margin,margin) -- \
       (margin,-margin)  withcolor white\t;\n\
       \t label(decimal t,(-margin,-margin)/2) ;\n\
       \t%%draw textext(\"cycle \" & decimal t) shifted (-margin,-margin)/2  ;\n\
       enddef ;\n\n\n\
       beginfig(0);\n\
       \t    mygrida (0) ;\n\
       endfig;\n\n\
       end.\n\n"
  in
  ()
