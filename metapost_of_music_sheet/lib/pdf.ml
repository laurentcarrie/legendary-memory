open Printf
module Log = Dolog.Log

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
      \includegraphics[width=\linewidth]{test2}
    \end{center}


\end{document}

|whatever}

let make_pdf path =
  let sheet : Sheet.sheet =
    Sheet.deserialize (In_channel.with_open_text path In_channel.input_all)
  in
  let _ = Log.info "%s:%d %s" __FILE__ __LINE__ sheet.title in
  (*  let (filename,fout) = Filename.open_temp_file "utest-test2" ".mp" in *)
  let filename = "test2.mp" in

  let write_mp () =
    let fout = open_out filename in
    let _ = Log.info "%s:%d name : %s" __FILE__ __LINE__ filename in
    let _ = Emitter.emit fout sheet "mps" "test2.mps" in
(*    let _ = Emitter.emit fout sheet "png" "test2.png" in *)
    let _ = close_out fout in
    (*    let _ = Log.info "%s:%d %s" __FILE__ __LINE__ sheet.title in *)
    (*    let data : string = *)
    (*      In_channel.with_open_text filename In_channel.input_all *)
    (*    in *)
    (*    let _ = Log.info "%s:%d %s" __FILE__ __LINE__ data in *)
    ()
  in
  let make_mps () =
    let _ = Unix.mkdir "mps" 0o740 in
    let status = Unix.system ("mpost --tex=latex " ^ filename) in
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("mpost wxited " ^ string_of_int i)
    | _ -> failwith "bad"
  in
  let write_tex () =
    let result : string = Jingoo.Jg_template.from_string maintex ~models:[] in
    let fout = open_out "test2.tex" in
    let _ = fprintf fout "%s\n" result in
    let _ = close_out fout in
    ()
  in

  let _make_pdf () =
    let status = Unix.system "lualatex test2" in
    let () =
      match status with
      | Unix.WEXITED 0 -> ()
      | Unix.WEXITED i -> failwith ("lualatex exited with code " ^ string_of_int i)
      | _ -> failwith "bad"
    in
    ()
  in

  let () = write_mp () in
  let () = make_mps () in
  let () = write_tex () in
  let () = _make_pdf () in
  let () = printf "test2 passed.\n" in
  ()
