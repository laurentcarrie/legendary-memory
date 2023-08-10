type chord = string [@@deriving yaml]
(**
blah blah blah
*)

type row = chord list [@@deriving yaml]
type section = { name : string; rows : row list } [@@deriving yaml]

type sheet = {
  title : string;
  authors : string list;
  (*  path : string; *)
  sections : section list;
}
[@@deriving yaml]

let deserialize str =
  match Yaml.of_string str with
  | Ok yaml_value -> (
      match sheet_of_yaml yaml_value with
      | Ok t -> t
      | Error (`Msg e) -> failwith ("Error - convert to sheet: " ^ e))
  | Error (`Msg e) -> failwith ("Error - parsing: " ^ e)

let serialize v =
  let yaml_structure = sheet_to_yaml v in
  match Yaml.to_string yaml_structure with
  | Ok s -> s
  | Error (`Msg e) -> failwith e
