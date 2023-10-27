module Log = Dolog.Log

let run ~message ~command =
  Log.info "%s:%d command : %s" Stdlib.__FILE__ Stdlib.__LINE__ command;
  let status = Unix.system command in
  let () =
    match status with
    | Unix.WEXITED 0 -> ()
    | Unix.WEXITED _ | _ -> failwith message
  in
  ()
