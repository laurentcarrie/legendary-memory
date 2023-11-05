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

(* let write_make_wav buildroot = *)
(* "(cd %s &&  fluidsynth --gain 4 -F %s /usr/share/sounds/sf2/FluidR3_GM.sf2 \ *)
   (*   %s.midi) && cp %s/%s %s" *)
(*  sheet.Sheet.tmpdir filename *)
(*    (Stdlib.Filename.remove_extension filename) *)
(*    sheet.Sheet.tmpdir filename filename *)
(*  *)
(* let write_make_pdf buildroot = *)
(*  let data = *)
(* {whatever|#!/bin/sh *)
   (*  *)
   (* set -e *)
   (* #set -x *)
   (* RED="\e[31m\e[47m" *)
   (* GREEN='\033[0;32m' *)
   (* CYAN='\033[0;36m' *)
   (* GREY="\e[37m" *)
   (* NC='\033[0m' # No Color *)
   (* workdir=$(dirname $(realpath $1)) *)
   (* printf "${GREY}building pdf in${NC} ${CYAN}$workdir$NC\n" *)
   (*  *)
   (* i="0" *)
   (* while [ $i -lt 4 ] *)
   (* do *)
   (* lualatex $1 1> make_pdf.stdout.log 2> make_pdf.stderr.log *)
   (* test -f main.log *)
   (* count=$(cat main.log | grep Rerun | wc --lines) *)
   (* if test "x$count" = "x0" ; then *)
   (*    break *)
   (* fi *)
   (* i=$[$i+1] *)
   (* done *)
   (*  *)
   (* printf "building pdf in ${GREEN}$workdir$NC done.\n" *)
   (*  *)
   (* |whatever} *)
(*  in *)
(*  *)
(*  let fout = Stdlib.open_out (buildroot ^ "/make_pdf.sh") in *)
(*  fprintf fout "%s" data; *)
(*  Stdlib.close_out fout *)
(*  *)
(* let write_make_lytex buildroot = *)
(*  let data = *)
(* {whatever|#!/bin/bash *)
   (*  *)
   (* set -e *)
   (* #set -x *)
   (*  *)
   (* RED="\e[31m\e[47m" *)
   (* GREEN='\033[0;32m\e[46m' *)
   (* CYAN='\033[0;36m' *)
   (* GREY="\e[37m" *)
   (* NC='\033[0m' # No Color *)
   (* workdir=$(dirname $(realpath $1)) *)
   (* printf "${GREY}building lilypond in${NC} ${CYAN}$workdir/$1$NC\n" *)
   (*  *)
   (*  *)
   (* lilypond-book $1 1> $1.lytex.stdout.log 2> $1.lytex.stderr.log || true *)
   (* lilypond-book $1 1>> $1.lytex.stdout.log 2>> $1.lytex.stderr.log *)
   (* printf "${GREY}building lilypond in${NC} ${GREEN}$workdir/$1$NC done\n" *)
   (*  *)
   (* |whatever} *)
(*  in *)
(*  *)
(*  let fout = Stdlib.open_out (buildroot ^ "/make_lytex.sh") in *)
(*  fprintf fout "%s" data; *)
(*  Stdlib.close_out fout *)
(*  *)
(* let write_make_mpost buildroot = *)
(*  let data = *)
(* {whatever|#!/bin/bash *)
   (*  *)
   (* set -e *)
   (* #set -x *)
   (*  *)
   (* mpost --tex=latex $1 1> $1.mpost.stdout.log 2> $1.mpost.stderr.log *)
   (*  *)
   (* |whatever} *)
(*  in *)
(*  *)
(*  let fout = Stdlib.open_out (buildroot ^ "/make-mpost.sh") in *)
(*  fprintf fout "%s" data; *)
(*  Stdlib.close_out fout *)

let relative_to dira dirb =
  let start = String.length dira + 1 in
  let l = String.length dirb - String.length dira - 1 in
  let ret = String.sub dirb ~pos:start ~len:l in
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
         ~f:(fun s ->
           let s2 = Stdlib.Filename.chop_extension s in
           s2 ^ ".output/" ^ s2 ^ ".tex")
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
  fprintf fout ".PHONY: pdf wav midi clean \n\n";

  fprintf fout
    "clean: \n\tbash  $(buildroot)/make_clean.sh \n\trm -rf %s \n\n\n"
    (List.fold_left
       ~f:(fun a b -> a ^ " " ^ b)
       ~init:""
       (List.map
          ~f:(fun s ->
            Stdlib.Filename.chop_extension s
            ^ ".lytex "
            ^ Stdlib.Filename.chop_extension s
            ^ ".tex")
          sheet.Sheet.lilypondfiles));
  fprintf fout "pdf : %s  \n\n" sheet.Sheet.pdf;
  fprintf fout
    "main.pdf : main.tex %s %s %s \n\tbash $(buildroot)/make_pdf.sh main \n\n"
    deps_pdf_of_ly deps_pdf_of_tex mps_files;

  fprintf fout "%s : main.pdf \n\tcp main.pdf $@ \n\n" sheet.Sheet.pdf;
  fprintf fout
    "%s : main.mp\n\
     \tmkdir -p mps\n\
     \tbash $(buildroot)/make_mpost.sh %s.mp  \n\n"
    mps_files "main";

  (* pdf of lytex *)
  List.iter
    ~f:(fun f ->
      let f = Stdlib.Filename.chop_extension f in
      fprintf fout
        "%s.output/%s.tex : %s.ly\n\tbash $(buildroot)/make_lytex.sh %s \n\n" f
        f f f)
    sheet.Sheet.lilypondfiles;

  (* wav files *)
  List.iter
    ~f:(fun f ->
      let f = Stdlib.Filename.chop_extension f in

      fprintf fout "midi : %s.midi\n\n" f;
      fprintf fout "wav : %s.wav\n\n" f;

      fprintf fout
        "%s.wav %s.midi : %s.ly \n\tbash $(buildroot)/make_wav.sh %s %s \n\n" f
        f f f
        (Scan.scan_ly sheet (f ^ ".ly")))
    sheet.Sheet.wavfiles;

  Stdlib.close_out fout;

  ()

let write_omakeroot buildroot rootdir =
  let fout = Stdlib.open_out (buildroot ^ "/OMakeroot") in
  let _ = rootdir in
  fprintf fout
    "srcdir = %s \n\
     prefix = delivery \n\
     buildroot = %s \n\
     DefineCommandVars() \n\
     public.srcdir = $(dir $(srcdir)) \n\
     CREATE_SUBDIRS=true \n\
     vmount(-c,$(srcdir),songs) \n\
     mkdir -p $(prefix) \n\
     .SUBDIRS: . \n"
    rootdir buildroot;

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
  fprintf fout
    "\n.PHONY: all install clean pdf delivery clean\n.SUBDIRS: \\%s\n\n" paths;

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

(* let write_lytexfiles sheet = *)
(*  let write_one filename = *)
(*    let _ = Log.info "write_lytexfiles %s" filename in *)
(*    let lytexfilename = *)
(*      sheet.Sheet.tmpdir ^ "/" *)
(*      ^ Stdlib.Filename.remove_extension filename *)
(*      ^ ".lytex" *)
(*    in *)
(*    let data = sprintf "\\lilypondfile{%s}\n" filename in *)
(*    Out_channel.with_open_text lytexfilename (fun t -> *)
(*        Out_channel.output_string t data) *)
(*  in *)
(*  List.iter ~f:(fun f -> write_one f) sheet.Sheet.lilypondfiles *)

let make_makefile buildroot rootdir =
  Log.info "rootdir %s" rootdir;
  mkdir_p buildroot;
  Pdf.generate_shlib buildroot;

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
  (*  let _ : unit list = *)
  (*    List.map ~f:(fun sheet -> write_lytexfiles sheet) sheets *)
  (*  in *)
  let (_ : unit list) =
    List.map ~f:(fun sheet -> Pdf.generate_texlib sheet) sheets
  in
  let (_ : unit list) =
    List.map ~f:(fun sheet -> Pdf.generate_lylib sheet) sheets
  in
  let (_ : unit list) = List.map ~f:(fun sheet -> Pdf.write_mp sheet) sheets in
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
