module Log = Dolog.Log

let () =
  Log.set_log_level Log.INFO;
  Log.set_output stdout;
  Log.color_on ();
  let yaml_filename = Array.get Sys.argv 1 in
  let _ = Metapost_of_music_sheet.Pdf.make_pdf yaml_filename in
  ()
