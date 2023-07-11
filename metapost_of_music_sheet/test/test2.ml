open Printf
module Log = Dolog.Log

let _ =
  Log.set_log_level Log.DEBUG;
  Log.set_output stdout;
  Log.color_on ();
  let path = "resources/music-sheet1.yml" in
  let _ = Totolib.Pdf.make_pdf path in
  let () = printf "test2 passed.\n" in
  ()
