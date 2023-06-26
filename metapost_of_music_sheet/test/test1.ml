(* open Metapost_of_music_sheet.Toto *)

let _ =
  let s : string = Array.get Sys.argv 0 in
  let _ = print_endline s in
  let x = Totolib.Toto.xxx 67 in
  let _ = print_endline (string_of_int x) in
  let data =
    In_channel.with_open_text "resources/music-sheet1.yml" In_channel.input_all
  in
  let sheet = Totolib.Sheet.deserialize data in
  let _ = print_endline sheet.title in
  ()
