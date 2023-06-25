(* let read_lines file = *)
(*  In_channel.with_open_text file In_channel.input_all *)
(*  |> Str.(split (regexp "\n")) *)

let _ =
  let s:string = Array.get Sys.argv 0 in
  let _ = print_endline s in
  let data = In_channel.with_open_text "resources/music-sheet1.yml" In_channel.input_all in
    print_endline data



