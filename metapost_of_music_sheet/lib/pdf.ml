open Base
open Printf
module Log = Dolog.Log

let normalized_title sheet =
  List.fold_left
    ~f:(fun ret (pattern, with_) ->
      String.substr_replace_all ret ~pattern ~with_)
    ~init:sheet.Sheet.title
    [ (" ", "_"); ("'", "_") ]

let generate_texlib sheet =
  let data = Tex_code.make_preamble in
  Out_channel.with_open_text (sheet.Sheet.tmpdir ^ "/preamble.tex") (fun t ->
      Out_channel.output_string t data)

let generate_lylib sheet =
  let data = Ly_code.make_macros in
  Out_channel.with_open_text (sheet.Sheet.tmpdir ^ "/macros.ly") (fun t ->
      Out_channel.output_string t data)

let generate_wavs_from_lilypond sheet =
  Log.info "WAV";
  let run_fluidsynth filename =
    let command =
      sprintf
        "(cd %s &&  fluidsynth --gain 4 -F %s \
         /usr/share/sounds/sf2/FluidR3_GM.sf2 %s.midi)"
        sheet.Sheet.tmpdir filename
        (Stdlib.Filename.remove_extension filename)
    in
    Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command;
    let status = Unix.system command in
    let () =
      match status with
      | Unix.WEXITED 0 -> ()
      | Unix.WEXITED i ->
          failwith ("fluidsynth exited with code " ^ Int.to_string i)
      | _ -> failwith "bad"
    in
    ()
  in
  List.iter ~f:run_fluidsynth sheet.Sheet.wavfiles

let generate_pdfs_from_lilypond sheet =
  let run_lilypond filename =
    let command =
      (*    sprintf "( cd $(dirname %s) && lualatex $(basename %s) )" path path *)
      sprintf "cp %s/%s %s/. && (cd %s  && lilypond %s) " sheet.Sheet.srcdir
        filename sheet.Sheet.tmpdir sheet.Sheet.tmpdir filename
    in
    Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command;
    let status = Unix.system command in
    let () =
      match status with
      | Unix.WEXITED 0 -> ()
      | Unix.WEXITED i ->
          failwith ("lilypond exited with code " ^ Int.to_string i)
      | _ -> failwith "bad"
    in
    ()
  in
  List.iter ~f:run_lilypond sheet.Sheet.lilypondfiles;
  ()

let pdf_of_tex sheet =
  let rec run () =
    let command =
      (*    sprintf "( cd $(dirname %s) && lualatex $(basename %s) )" path path *)
      sprintf "(cd %s && lualatex main.tex) && cp %s/main.pdf %s.pdf "
        sheet.Sheet.tmpdir sheet.Sheet.tmpdir (normalized_title sheet)
    in
    Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command;
    let status = Unix.system command in
    let () =
      match status with
      | Unix.WEXITED 0 -> ()
      | Unix.WEXITED i ->
          failwith ("lualatex exited with code " ^ Int.to_string i)
      | _ -> failwith "bad"
    in
    let logdata =
      In_channel.with_open_text
        (sheet.Sheet.tmpdir ^ "/main.log")
        In_channel.input_all
    in
    match String.substr_index logdata ~pattern:"Rerun" with
    | Some _ -> run ()
    | None -> ()
  in

  run ()

let mps_of_mp sheet =
  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let _ = sheet in
  let mps_dir = sheet.Sheet.tmpdir ^ "/mps" in
  let _ =
    Log.info "%s:%d file exists %s : %b" Stdlib.__FILE__ Stdlib.__LINE__ mps_dir
      (Stdlib.Sys.file_exists mps_dir)
  in
  let _ =
    try
      let _ = Unix.mkdir mps_dir 0o740 in
      ()
    with _ -> ()
  in
  let command =
    sprintf "( cd %s && mpost --tex=latex main.mp )" sheet.Sheet.tmpdir
  in
  let _ =
    Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command
  in
  let status = Unix.system command in
  let _ =
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("mpost wxited " ^ Int.to_string i)
    | _ -> failwith "bad"
  in
  ()

let maintex : string =
  {whatever|
\documentclass[11pt]{article}
\usepackage{graphicx}
\usepackage{unicode-math}
    \setmainfont{XITS}
    \setmathfont{XITS Math}
\usepackage{luamplib}
\usepackage{xcolor}
\begin{document}
\section*{section 1}
    %\begin{center}
      \includegraphics{ {{mpsname}} }
    %\end{center}


\end{document}

|whatever}

(* let tex_of_mps sheet = *)
(*  let _ = sheet in *)
(*  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in *)
(*  let result : string = *)
(*    Jingoo.Jg_template.from_string maintex *)
(*      ~models:[ ("mpsname", Jingoo.Jg_types.Tstr "mps/main-0") ] *)
(*  in *)
(*  let fout = open_out "main.tex" in *)
(*  let _ = fprintf fout "%s\n" result in *)
(*  let _ = close_out fout in *)
(*  () *)

let make_pdf yaml_filename =
  Log.info "deserialize %s" yaml_filename;
  let input_sheet : Input_sheet.sheet =
    Input_sheet.deserialize
      (In_channel.with_open_text yaml_filename In_channel.input_all)
  in
  let sheet =
    Input_sheet.sheet_of_input input_sheet
      (Stdlib.Filename.dirname yaml_filename)
  in
  Log.info "%s:%d %s" Stdlib.__FILE__ Stdlib.__LINE__ sheet.Sheet.title;
  let () =
    try
      Unix.mkdir sheet.Sheet.tmpdir 0o740;
      ()
    with _ -> ()
  in
  (*  let (filename,fout) = Filename.open_temp_file "utest-test2" ".mp" in *)
  let mp_filename = sheet.Sheet.tmpdir ^ "/main.mp" in

  (*  let mps_filename = sprintf "%s.mps" sheet.Sheet.path in *)
  let write_mp () =
    let fout = Stdio.Out_channel.create mp_filename in
    Log.info "%s:%d writing name : %s" Stdlib.__FILE__ Stdlib.__LINE__
      mp_filename;
    let _ = Emitter.emit fout sheet "mps" (sheet.Sheet.tmpdir ^ "/main.mps") in
    let _ = Stdio.Out_channel.close fout in
    ()
  in

  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let () = write_mp () in
  let () = mps_of_mp sheet in

  let _ =
    List.iter
      ~f:(fun texname ->
        try
          let texpath = Stdlib.Filename.dirname yaml_filename ^ "/" ^ texname in
          Log.info "%s:%d reading texfile : %s" Stdlib.__FILE__ Stdlib.__LINE__
            texpath;
          let data = In_channel.with_open_text texpath In_channel.input_all in
          let () =
            Out_channel.with_open_text
              (sheet.Sheet.tmpdir ^ "/" ^ texname)
              (fun t -> Out_channel.output_string t data)
          in
          ()
        with e ->
          let () = Stdlib.Printexc.print_backtrace Stdlib.stdout in
          Log.info "%s:%d found file NOT FOUND %s" Stdlib.__FILE__
            Stdlib.__LINE__
            (Stdlib.Printexc.to_string e);
          failwith ("file not found " ^ texname))
      sheet.Sheet.texfiles
  in

  generate_texlib sheet;
  generate_lylib sheet;
  generate_pdfs_from_lilypond sheet;
  generate_wavs_from_lilypond sheet;
  pdf_of_tex sheet;
  (*  let () = printf "test2 passed.\n" in *)
  ()
