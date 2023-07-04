open Printf
module Log = Dolog.Log

let maintex : string =
  {whatever|
\documentclass[11pt]{article}
\usepackage{graphicx}

\begin{document}

    \begin{center}
      \includegraphics[width=\linewidth]{test2}
    \end{center}


\end{document}

|whatever}

let _ =
  Log.set_log_level Log.DEBUG;
  Log.set_output stdout;
  Log.color_on ();

  let sheet : Totolib.Sheet.sheet =
    Totolib.Sheet.deserialize
      (In_channel.with_open_text "resources/music-sheet1.yml"
         In_channel.input_all)
  in
  let _ = Log.info "%s:%d %s" __FILE__ __LINE__ sheet.title in
  (*  let (filename,fout) = Filename.open_temp_file "utest-test2" ".mp" in *)
  let filename = "test2.mp" in

  let write_mp () =
    let fout = open_out filename in
    let _ = Log.info "%s:%d name : %s" __FILE__ __LINE__ filename in
    let _ = Totolib.Emitter.emit fout sheet "mps" "test2.mps" in
    let _ = close_out fout in
    let _ = Log.info "%s:%d %s" __FILE__ __LINE__ sheet.title in
    let data : string =
      In_channel.with_open_text filename In_channel.input_all
    in
    let _ = Log.info "%s:%d %s" __FILE__ __LINE__ data in
    ()
  in
  let make_mps () =
    let _ = Unix.mkdir "mps" 0o740 in
    let status = Unix.system ("mpost --tex=latex " ^ filename) in
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("wxited " ^ string_of_int i)
    | _ -> failwith "bad"
  in
  let write_tex () =
    let result : string = Jingoo.Jg_template.from_string maintex ~models:[] in
    let fout = open_out "test2.tex" in
    let _ = fprintf fout "%s\n" result in
    let _ = close_out fout in
    ()
  in

  let make_pdf () =
    let status = Unix.system "lualatex test2" in
    let () =
      match status with
      | Unix.WEXITED 0 -> ()
      | Unix.WEXITED i -> failwith ("wexited " ^ string_of_int i)
      | _ -> failwith "bad"
    in
    ()
  in

  let () = write_mp () in
  let () = make_mps () in
  let () = write_tex () in
  let () = make_pdf () in
  let () = printf "test2 passed.\n" in
  ()
