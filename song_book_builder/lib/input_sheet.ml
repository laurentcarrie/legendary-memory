module Log = Dolog.Log

type row = string list [@@deriving yaml]
type section = { name : string; rows : row list } [@@deriving yaml]

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

let pdf_name_of_input input =
  let s = input.author ^ "--@--" ^ input.title in
  let s =
    String.map
      (fun x -> match x with ' ' | '\'' | '/' -> '-' | '.' -> '-' | _ -> x)
      s
  in
  let s = String.lowercase_ascii s in
  let s = s ^ ".pdf" in
  s

let sheet_of_input ~input ~srcdir ~tmpdir =
  let row_of_row ~row ~transpose =
    {
      Sheet.bars =
        List.map
          (fun s ->
            {
              Sheet.chords =
                List.map
                  (fun chord -> Transpose.transpose_chord ~chord ~transpose)
                  (String.split_on_char ' ' s);
            })
          row;
    }
  in

  let transpose =
    match (input.transpose_from, input.transpose_to) with
    | None, None -> None
    | Some i, Some j -> Some (i, j)
    | _ -> failwith "bad transpose_from or transpose_to fields"
  in

  let output =
    {
      Sheet.title = input.title;
      author = input.author;
      pdf = pdf_name_of_input input;
      transpose;
      sections =
        List.map
          (fun section ->
            {
              Sheet.name = section.name;
              rows =
                List.map (fun row -> row_of_row ~row ~transpose) section.rows;
            })
          input.sections;
      cell_width = input.cell_width;
      cell_height = input.cell_height;
      chord_glyph_scale = input.chord_glyph_scale;
      texfiles = List.append input.texfiles [ "main.tex" ];
      lilypondfiles = input.lilypondfiles;
      wavfiles = input.wavfiles;
      tmpdir;
      srcdir;
    }
  in
  (*  let () = Log.info "%s" output in *)
  output
