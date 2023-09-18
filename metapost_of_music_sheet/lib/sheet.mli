(**
 sheet data model
*)

type chord = string
type row = chord list
type section = { name : string; rows : row list }

type sheet = {
  title : string;
  authors : string list;
  (*  path : string; *)
  sections : section list;
  cell_width : float;
  cell_height : float;
  chord_glyph_scale : float;
}

val deserialize : string -> sheet
(**
    deserialize a string to yaml
*)

val serialize : sheet -> string