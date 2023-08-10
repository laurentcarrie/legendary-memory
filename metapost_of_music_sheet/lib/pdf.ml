open Printf
module Log = Dolog.Log

let pdf_of_tex sheet =
  let _ = sheet in
  let _ = Log.info "%s:%d" __FILE__ __LINE__ in
  let command =
    (*    sprintf "( cd $(dirname %s) && lualatex $(basename %s) )" path path *)
    "lualatex main.tex"
  in
  let _ = Log.info "%s:%d command : %s" __FILE__ __LINE__ command in
  let status = Unix.system command in
  let () =
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("lualatex exited with code " ^ string_of_int i)
    | _ -> failwith "bad"
  in
  ()

let mps_of_mp sheet =
  let _ = Log.info "%s:%d" __FILE__ __LINE__ in
  let _ = sheet in
  let mps_dir = "mps" in
  let _ = if not (Sys.file_exists mps_dir) then Unix.mkdir mps_dir 0o740 in
  let command = sprintf "( mpost --tex=latex main.mp )" in
  let _ = Log.info "%s:%d command : %s" __FILE__ __LINE__ command in
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
\section{section 1}
    \begin{center}
      \includegraphics[width=\linewidth]{ {{mpsname}} }
    \end{center}


\end{document}

|whatever}

let tex_of_mps sheet =
  let _ = sheet in
  let _ = Log.info "%s:%d" __FILE__ __LINE__ in
  let result : string =
    Jingoo.Jg_template.from_string maintex
      ~models:[ ("mpsname", Jingoo.Jg_types.Tstr "mps/main-0") ]
  in
  let fout = open_out "main.tex" in
  let _ = fprintf fout "%s\n" result in
  let _ = close_out fout in
  ()

let make_pdf yaml_filename =
  let _ = Log.info "%s:%d" __FILE__ __LINE__ in
  let _ = Log.info "pwd : %s" (Sys.getcwd ()) in
  let _ = Log.info "deserialize %s" yaml_filename in
  let sheet : Sheet.sheet =
    Sheet.deserialize
      (In_channel.with_open_text yaml_filename In_channel.input_all)
  in
  let _ = Log.info "%s:%d %s" __FILE__ __LINE__ sheet.title in
  (*  let (filename,fout) = Filename.open_temp_file "utest-test2" ".mp" in *)
  let mp_filename = "main.mp" in

  (*  let mps_filename = sprintf "%s.mps" sheet.Sheet.path in *)
  let write_mp () =
    let fout = open_out mp_filename in
    let _ = Log.info "%s:%d writing name : %s" __FILE__ __LINE__ mp_filename in
    let _ = Emitter.emit fout sheet "mps" "main.mps" in
    let _ = close_out fout in
    ()
  in

  let _ = Log.info "%s:%d" __FILE__ __LINE__ in
  let () = write_mp () in
  let () = mps_of_mp sheet in
  let () = tex_of_mps sheet in
  let () = pdf_of_tex sheet in
  (*  let () = printf "test2 passed.\n" in *)
  ()
