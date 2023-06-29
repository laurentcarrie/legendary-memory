module Log = Dolog.Log

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
  let filename = "test.mp" in
  let fout = open_out filename in
  let _ = Log.info "%s:%d name : %s" __FILE__ __LINE__ filename in
  let _ = Totolib.Emitter.emit fout sheet "png" in
  let _ = close_out fout in
  let _ = Log.info "%s:%d %s" __FILE__ __LINE__ sheet.title in
  let data : string = In_channel.with_open_text filename In_channel.input_all in
  let _ = Log.info "%s:%d %s" __FILE__ __LINE__ data in
  let _ = Unix.mkdir "mps" 0o740 in
  let status = Unix.system ("mpost --tex=latex " ^ filename) in
  let () =
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED i -> failwith ("wxited " ^ string_of_int i)
    | _ -> failwith "bad"
  in
  let _ = failwith "test" in
      ()
