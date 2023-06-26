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
  let _ = Totolib.Emitter.emit stdout sheet in
  ()
