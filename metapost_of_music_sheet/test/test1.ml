module Log = Dolog.Log

let _ =
  Log.set_log_level Log.DEBUG;
  Log.set_output stdout;
  Log.color_on ();
  let s : string = Array.get Sys.argv 0 in
  let _ = print_endline s in
  let x = Totolib.Toto.xxx 67 in
  let _ = print_endline (string_of_int x) in
  let data =
    In_channel.with_open_text "resources/music-sheet1.yml" In_channel.input_all
  in
  let sheet = Totolib.Sheet.deserialize data in
  let _ = Log.info "%s" sheet.Totolib.Sheet.title in
  ()
