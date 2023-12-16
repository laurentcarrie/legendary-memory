(**
 sheet data model
*)

type chord = string
type bar = { chords : chord list }
type row = { bars : bar list }
type section = { name : string; rows : row list }

type sheet = {
  title : string;
  author : string;
  pdf : string;
  transpose : (string * string) option;
  sections : section list;
  cell_width : float;
  cell_height : float;
  chord_glyph_scale : float;
  texfiles : string list;
  lilypondfiles : string list;
  wavfiles : string list;
  srcdir : string;
  tmpdir : string;
}
