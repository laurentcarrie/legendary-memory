open Printf
module Log = Dolog.Log

let pdf_of_tex sheet =
  let _ = sheet in
  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let command =
    (*    sprintf "( cd $(dirname %s) && lualatex $(basename %s) )" path path *)
    "lualatex main.tex"
  in
  let _ =
    Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command
  in
  let status = Unix.system command in
  let () =
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("lualatex exited with code " ^ Int.to_string i)
    | _ -> failwith "bad"
  in
  ()

let mps_of_mp sheet =
  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let _ = sheet in
  let mps_dir = "mps" in
  let _ =
    Log.info "%s:%d file exists %s : %b" Stdlib.__FILE__ Stdlib.__LINE__ mps_dir
      (Sys.file_exists mps_dir)
  in
  let _ =
    try
      let _ = Unix.mkdir mps_dir 0o740 in
      ()
    with _ -> ()
  in
  let command = sprintf "( mpost --tex=latex main.mp )" in
  let _ =
    Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command
  in
  let status = Unix.system command in
  let _ =
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("mpost wxited " ^ string_of_int i)
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

let tex_of_mps sheet =
  let _ = sheet in
  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let result : string =
    Jingoo.Jg_template.from_string maintex
      ~models:[ ("mpsname", Jingoo.Jg_types.Tstr "mps/main-0") ]
  in
  let fout = open_out "main.tex" in
  let _ = fprintf fout "%s\n" result in
  let _ = close_out fout in
  ()

let make_pdf yaml_filename =
  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let _ = Log.info "pwd : %s" (Sys.getcwd ()) in
  let _ = Log.info "deserialize %s" yaml_filename in
  let input_sheet : Input_sheet.sheet =
    Input_sheet.deserialize
      (In_channel.with_open_text yaml_filename In_channel.input_all)
  in
  let sheet = Input_sheet.sheet_of_input input_sheet in
  let _ = Log.info "%s:%d %s" Stdlib.__FILE__ Stdlib.__LINE__ sheet.title in
  (*  let (filename,fout) = Filename.open_temp_file "utest-test2" ".mp" in *)
  let mp_filename = "main.mp" in

  (*  let mps_filename = sprintf "%s.mps" sheet.Sheet.path in *)
  let write_mp () =
    let fout = open_out mp_filename in
    let _ =
      Log.info "%s:%d writing name : %s" Stdlib.__FILE__ Stdlib.__LINE__
        mp_filename
    in
    let _ = Emitter.emit fout sheet "mps" "main.mps" in
    let _ = close_out fout in
    ()
  in

  let _ = Log.info "%s:%d" Stdlib.__FILE__ Stdlib.__LINE__ in
  let () = write_mp () in
  let () = mps_of_mp sheet in

  let texpath = Filename.dirname yaml_filename ^ "/main.tex" in
  let _ =
    Log.info "%s:%d dirname : %s" Stdlib.__FILE__ Stdlib.__LINE__ texpath
  in

  let _ =
    try
      let data = In_channel.with_open_text texpath In_channel.input_all in
      let () =
        Log.info "%s:%d found file %s" Stdlib.__FILE__ Stdlib.__LINE__ texpath
      in
      let () =
        Out_channel.with_open_text "main.tex" (fun t ->
            Out_channel.output_string t data)
      in
      ()
    with _ ->
      Log.info "%s:%d found file NOT FOUND %s" Stdlib.__FILE__ Stdlib.__LINE__
        texpath;
      tex_of_mps sheet;
      ()
  in

  let () = pdf_of_tex sheet in
  (*  let () = printf "test2 passed.\n" in *)
  ()
