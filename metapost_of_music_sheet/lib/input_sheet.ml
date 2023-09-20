module Log = Dolog.Log

type row = string list [@@deriving yaml]
type section = { name : string; rows : row list } [@@deriving yaml]

type sheet = {
  title : string;
  authors : string list;
  (*  path : string; *)
  sections : section list;
  cell_width : float;
  cell_height : float;
  chord_glyph_scale : float;
  texfiles : string list;
  lilypondfiles : string list;
  wavfiles : string list;
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

let sheet_of_input input srcdir =
  let row_of_row row =
    {
      Sheet.bars =
        List.map (fun s -> { Sheet.chords = String.split_on_char ' ' s }) row;
    }
  in

  let output =
    {
      Sheet.title = input.title;
      authors = input.authors;
      sections =
        List.map
          (fun section ->
            {
              Sheet.name = section.name;
              rows = List.map row_of_row section.rows;
            })
          input.sections;
      cell_width = input.cell_width;
      cell_height = input.cell_height;
      chord_glyph_scale = input.chord_glyph_scale;
      texfiles = List.append input.texfiles [ "main.tex" ];
      lilypondfiles = input.lilypondfiles;
      wavfiles = input.wavfiles;
      tmpdir = "tmp";
      srcdir;
    }
  in
  (*  let () = Log.info "%s" output in *)
  output
