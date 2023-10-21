open Printf
open Base
module Log = Dolog.Log

let buildroot = "build-songs"

let mkdir_p p =
  let rec attempt pp =
    try if not (Stdlib.Sys.file_exists pp) then Unix.mkdir pp 0o740 else ()
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

let write_omakefile rootdir yaml_filename =
  let _ = Log.info "write omakefile %s %s" rootdir yaml_filename in
  let p = buildroot ^ "/" ^ yaml_filename in
  let makefile_name =
    buildroot ^ "/" ^ Stdlib.Filename.dirname yaml_filename ^ "/OMakefile"
  in
  let _ = Log.info "omakefile is %s" makefile_name in
  let d = Stdlib.Filename.dirname p in
  let _ = Log.info "dirname is %s" d in
  let _ = mkdir_p d in

  let input_sheet : Input_sheet.sheet =
    Input_sheet.deserialize
      (In_channel.with_open_text yaml_filename In_channel.input_all)
  in
  let sheet =
    Input_sheet.sheet_of_input input_sheet
      (Stdlib.Filename.dirname yaml_filename)
  in

  let fout = Stdlib.open_out makefile_name in
  fprintf fout ".PHONY: pdf clean \n\n";
  fprintf fout "pdf : %s \n\n" sheet.Sheet.pdf;
  fprintf fout "%s : %s\n\tlualatex main.tex\n\tmv main.pdf $@\n\n"
    sheet.Sheet.pdf "main.tex";

  Stdlib.close_out fout;

  ()

let write_omakeroot buildroot rootdir =
  let fout = Stdlib.open_out (buildroot ^ "/OMakeroot") in
  let _ = rootdir in
  fprintf fout
    "srcdir = %s \n\
     prefix = xx \n\
     DefineCommandVars() \n\
     public.srcdir = $(dir $(srcdir)) \n\
     CREATE_SUBDIRS=true \n\
     vmount(-c,$(srcdir),songs) \n\
     mkdir -p $(prefix) \n\
     .SUBDIRS: . \n"
    "/home/laurent/work/legendary-memory/songs";

  Stdlib.close_out fout

let write_top_omakefile l =
  let fout = Stdlib.open_out (buildroot ^ "/OMakefile") in
  let paths =
    List.fold_left
      ~f:(fun acc s -> sprintf "%s\n\t%s \\" acc (Stdlib.Filename.dirname s))
      ~init:"" l
  in
  fprintf fout "\n.PHONY: all install clean pdf\n.SUBDIRS: \\%s" paths;
  Stdlib.close_out fout

let make_makefile rootdir =
  Log.info "rootdir %s" rootdir;
  mkdir_p buildroot;
  let l : string list = get_all_songs rootdir in
  let () = write_omakeroot buildroot rootdir in
  let () = write_top_omakefile l in
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
