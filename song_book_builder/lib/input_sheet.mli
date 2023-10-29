(**
 sheet data model
*)

type row = string list
type section = { name : string; rows : row list }

type sheet = {
  title : string;
  author : string;
  transpose_from : string option;
  transpose_to : string option;
  (*  path : string; *)
  sections : section list;
  cell_width : float;
  cell_height : float;
  chord_glyph_scale : float;
  texfiles : string list;
  lilypondfiles : string list;
  wavfiles : string list;
}

val deserialize : string -> sheet
(**
    deserialize a string to yaml
*)

val serialize : sheet -> string

val sheet_of_input :
  input:sheet -> srcdir:string -> tmpdir:string -> Sheet.sheet
