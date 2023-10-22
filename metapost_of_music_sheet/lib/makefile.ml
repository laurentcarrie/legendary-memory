open Printf
open Base
module Log = Dolog.Log

let mkdir_p p =
  let rec attempt pp =
    try if not (Stdlib.Sys.file_exists pp) then Unix.mkdir pp 0o740 else ()
    with _ ->
      attempt (Stdlib.Filename.dirname pp);
      attempt pp
  in
  attempt p

let relative_to dira dirb =
  let _ = Log.info "dira %s" dira in
  let _ = Log.info "dirb %s" dirb in
  let start = String.length dira + 1 in
  let l = String.length dirb - String.length dira - 1 in
  let ret = String.sub dirb ~pos:start ~len:l in
  let _ = Log.info "relative_to : %s" ret in
  ret

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

let builddir_of_yaml_filename buildroot rootdir yaml_filename =
  let rel = relative_to rootdir yaml_filename in
  let relpath = Stdlib.Filename.dirname rel in
  let _ = Log.info "rootdir : %s" rootdir in
  let _ = Log.info "yaml_filename : %s" yaml_filename in
  let _ = Log.info "relative : %s" relpath in
  let _ = Log.info "buildroot : %s" buildroot in
  buildroot ^ "/songs/" ^ relpath

let write_omakefile sheet =
  let makefile_name = sheet.Sheet.tmpdir ^ "/OMakefile" in
  (*  let _ = Log.info "omakefile is %s" makefile_name in *)
  let d = Stdlib.Filename.dirname makefile_name in
  (*  let _ = Log.info "dirname is %s" d in *)
  let _ = mkdir_p d in

  let deps_pdf_of_ly =
    List.fold_left
      ~f:(fun acc s -> acc ^ " " ^ s)
      ~init:""
      (List.map
         ~f:(fun s -> Stdlib.Filename.chop_extension s ^ ".tex")
         sheet.Sheet.lilypondfiles)
  in

  let deps_pdf_of_tex =
    List.fold_left
      ~f:(fun acc s -> acc ^ " " ^ s)
      ~init:""
      (List.map ~f:(fun s -> s) sheet.Sheet.texfiles)
  in

  let mps_files =
    List.fold_left
      ~f:(fun acc s -> acc ^ s ^ " ")
      ~init:""
      (List.mapi
         ~f:(fun i _ -> sprintf "mps/main-%d.mps" i)
         (*         sheet.Sheet.sections *)
         [ 0 ])
  in

  let fout = Stdlib.open_out makefile_name in
  fprintf fout ".PHONY: pdf clean \n\n";
  fprintf fout "pdf : %s  \n\n" sheet.Sheet.pdf;
  fprintf fout
    "%s : %s %s %s %s \n\
     \tlualatex main.tex || true\n\
     \tlualatex main.tex || true\n\
     \tlualatex main.tex\n\
     \tmv main.pdf $@ \n\n"
    sheet.Sheet.pdf "main.tex" deps_pdf_of_ly deps_pdf_of_tex mps_files;

  fprintf fout "%s : main.mp\n\tmkdir -p mps\n\tmpost --tex=latex main.mp\n\n"
    mps_files;

  (* pdf of lytex *)
  List.iter
    ~f:(fun f ->
      let f = Stdlib.Filename.chop_extension f in
      fprintf fout
        "%s.tex : %s.lytex %s.ly\n\
         \tlilypond-book --pdf %s.lytex || true \n\
         \tlilypond-book --pdf %s.lytex \n\n"
        f f f f f)
    sheet.Sheet.lilypondfiles;

  Stdlib.close_out fout;

  ()

let write_omakeroot buildroot rootdir =
  let fout = Stdlib.open_out (buildroot ^ "/OMakeroot") in
  let _ = rootdir in
  fprintf fout
    "srcdir = %s \n\
     prefix = delivery \n\
     DefineCommandVars() \n\
     public.srcdir = $(dir $(srcdir)) \n\
     CREATE_SUBDIRS=true \n\
     vmount(-c,$(srcdir),songs) \n\
     mkdir -p $(prefix) \n\
     .SUBDIRS: . \n"
    rootdir;

  Stdlib.close_out fout

let write_top_omakefile buildroot sheets =
  let fout = Stdlib.open_out (buildroot ^ "/OMakefile") in
  let paths =
    List.fold_left
      ~f:(fun acc sheet ->
        let _ = Log.info "sheet tmpdir %s" sheet.Sheet.tmpdir in
        let r = relative_to buildroot sheet.Sheet.tmpdir in

        sprintf "%s\n\t%s \\" acc r)
      ~init:"" sheets
  in
  fprintf fout "\n.PHONY: all install clean pdf delivery\n.SUBDIRS: \\%s\n\n"
    paths;

  fprintf fout "delivery:\\\n";
  fprintf fout "%s\n"
    (Util.join
       (List.map
          ~f:(fun sheet ->
            "\t"
            ^ relative_to buildroot sheet.Sheet.tmpdir
            ^ "/" ^ sheet.Sheet.pdf)
          sheets)
       " \\\n");
  fprintf fout "\trm -rf delivery \n\tmkdir delivery\n\tcp $^ delivery/. \n";

  Stdlib.close_out fout

let write_lytexfiles sheet =
  let write_one filename =
    let _ = Log.info "write_lytexfiles %s" filename in
    let lytexfilename =
      sheet.Sheet.tmpdir ^ "/"
      ^ Stdlib.Filename.remove_extension filename
      ^ ".lytex"
    in
    let data = sprintf "\\lilypondfile{%s}\n" filename in
    Out_channel.with_open_text lytexfilename (fun t ->
        Out_channel.output_string t data)
  in
  List.iter ~f:(fun f -> write_one f) sheet.Sheet.lilypondfiles

let make_makefile buildroot rootdir =
  Log.info "rootdir %s" rootdir;
  mkdir_p buildroot;
  let list_of_yamlfiles : string list = get_all_songs rootdir in
  let sheets =
    List.map
      ~f:(fun yaml_filename ->
        let input_sheet : Input_sheet.sheet =
          Input_sheet.deserialize
            (In_channel.with_open_text yaml_filename In_channel.input_all)
        in
        let sheet =
          Input_sheet.sheet_of_input ~input:input_sheet
            ~srcdir:(Stdlib.Filename.dirname yaml_filename)
            ~tmpdir:(builddir_of_yaml_filename buildroot rootdir yaml_filename)
        in
        sheet)
      list_of_yamlfiles
  in
  let () = write_omakeroot buildroot rootdir in
  let () = write_top_omakefile buildroot sheets in
  let _ = List.map ~f:(fun sheet -> write_omakefile sheet) sheets in
  let _ : unit list =
    List.map ~f:(fun sheet -> write_lytexfiles sheet) sheets
  in
  let _ : unit list =
    List.map ~f:(fun sheet -> Pdf.generate_texlib sheet) sheets
  in
  let _ : unit list = List.map ~f:(fun sheet -> Pdf.write_mp sheet) sheets in
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
