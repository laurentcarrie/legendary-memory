(**
 sheet data model
*)

type chord = string
type row = chord list
type section = { name : string; rows : row list }
type sheet = {
  title : string;
  authors : string list;
  path : string;
  sections : section list;
}

(**
    deserialize a string to yaml
*)
val deserialize : string -> sheet

val serialize : sheet -> string
