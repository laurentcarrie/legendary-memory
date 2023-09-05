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
  cell_width : int;
  cell_height : int;
}

val deserialize : string -> sheet
(**
    deserialize a string to yaml
*)

val serialize : sheet -> string
