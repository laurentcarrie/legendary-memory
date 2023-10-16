open Base
module Log = Dolog.Log

let buildroot = "build-songs"

let mkdir_p p =
  let rec attempt pp =
    try Unix.mkdir pp 0o740
    with _ ->
      attempt (Stdlib.Filename.dirname pp);
      attempt pp
  in
  attempt p

let get_all_songs rootdir =
  let rec scan f acc =
    let b : string = Stdlib.Filename.basename f in
    if Stdlib.Sys.is_directory f then
      List.fold_left
        ~f:(fun acc ff -> scan (f ^ "/" ^ ff) acc)
        ~init:acc
        (Array.to_list (Stdlib.Sys.readdir f))
      (*    else if Stdlib.Filename.basename f = "song.yml" then f :: acc *)
    else if String.compare b "song.yml" = 0 then f :: acc
    else acc
  in
  scan rootdir []

let write_omakefile rootdir relsong =
  let _ = Log.info "write omakefile %s %s" rootdir relsong in
  let p = buildroot ^ "/" ^ relsong in
  let d = Stdlib.Filename.dirname p in
  let _ = Log.info "dirname is %s" d in
  let _ = mkdir_p d in
  ()

let make_makefile rootdir =
  Log.info "rootdir %s" rootdir;
  let l : string list = get_all_songs rootdir in
  let _ = List.map ~f:(fun f -> write_omakefile rootdir f) l in
  ()

(*  let input_sheet : Input_sheet.sheet = *)
(*    Input_sheet.deserialize *)
(*      (In_channel.with_open_text yaml_filename In_channel.input_all) *)
(*  in *)
(*  let sheet = *)
(*    Input_sheet.sheet_of_input input_sheet *)
(*      (Stdlib.Filename.dirname yaml_filename) *)
(*  in *)
(*  Log.info "%s:%d %s" Stdlib.__FILE__ Stdlib.__LINE__ sheet.Sheet.title; *)
(*  let data = "hello world" in *)
(*  Out_channel.with_open_text ( "build-songs/Makefile") (fun t -> *)
(*      Out_channel.output_string t data) *)
