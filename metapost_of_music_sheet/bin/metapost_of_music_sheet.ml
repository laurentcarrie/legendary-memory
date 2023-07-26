module Log = Dolog.Log

(* (* Define a record *) *)
(* (* `[@@deriving yaml]` generates a bunch of functions, one being `book_to_yaml` to convert the record into a Yaml type, another `book_of_yaml` to convert Yaml type to record *) *)
(* type book = { *)
(*  title: string; *)
(*  authors: string list *)
(* }[@@deriving yaml] *)
(*  *)
(* let serialize_book  v = *)
(*  (* `book_to_yaml` converts from record to `yaml res` where res is a Result *) *)
(*  let yaml_structure = book_to_yaml v in *)
(*  (* `to_string` converts from a `yaml` type'ed data structure to string *) *)
(*  match Yaml.to_string yaml_structure with *)
(*  | Ok s -> *)
(*    print_endline ("Serialize:"); *)
(*    print_endline (s) *)
(*  | Error (`Msg e) -> print_endline e *)
(*  *)
(* let deserialize_book str  = *)
(*  (* `of_string converts from string to a `yaml res` data structure, where `res` is Result *) *)
(*  match Yaml.of_string str with *)
(*  | Ok yaml_value -> *)
(*    (* `book_of_yaml` is generated by `[@@deriving yaml]` *) *)
(*    (* `book_of_yaml` converts from `yaml` type to `book res` where res is Result  *) *)
(*    (match book_of_yaml yaml_value with *)
(*    | Ok t -> *)
(*      print_endline ("Deserialize:"); *)
(*      print_endline ("Title: " ^ t.title); *)
(*      print_endline ("Authors: " ^ String.concat ", " t.authors); *)
(*    | Error `Msg e -> print_endline ("Error - convert to book: " ^ e)) *)
(*  | Error `Msg e -> print_endline ("Error - parsing: " ^ e) *)

let () =
  Log.set_log_level Log.DEBUG;
  Log.set_output stdout;
  Log.color_on ();
  let yaml_filename = Array.get Sys.argv 1 in
  let _ = Metapost_of_music_sheet.Pdf.make_pdf yaml_filename in
  ()
