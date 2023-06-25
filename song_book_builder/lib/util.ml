open Base

let join (l : string list) sep : string =
  match l with
  | [] -> ""
  | hd :: [] -> hd
  | hd :: tl -> List.fold_left ~f:(fun s acc -> acc ^ sep ^ s) ~init:hd tl
