let _ =
  let s:string = Array.get Sys.argv 0 in
  let _ = print_endline s in
  let x = Metapost_of_music_sheet.Toto.xxx 78 in
(*  let data = In_channel.with_open_text "resources/music-sheet1.yml" In_channel.input_all in *)
(*  let sheet = Metapost_of_music_sheet.Sheet.deserialize_sheet data in *)
(*    print_endline sheet *)
()
