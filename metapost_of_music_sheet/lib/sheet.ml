type sheet = {
  title: string;
  authors: string list
}[@@deriving yaml]

let deserialize_sheet str  =
  match Yaml.of_string str with
  | Ok yaml_value ->
    (match sheet_of_yaml yaml_value with
    | Ok t -> t
    | Error `Msg e -> failwith ("Error - convert to sheet: " ^ e))
  | Error `Msg e -> failwith ("Error - parsing: " ^ e)

