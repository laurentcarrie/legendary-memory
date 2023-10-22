open Base
open Printf
module Log = Dolog.Log
open Helpers

let of_lilypond sheet =
  let run ~filename =
    let () =
      match Stdlib.Filename.check_suffix filename "ly" with
      | true -> ()
      | false -> failwith (sprintf "%s should have extension .ly" filename)
    in

    let lytexfilename = Stdlib.Filename.remove_extension filename ^ ".lytex" in
    let data = sprintf "\\lilypondfile{%s}\n" filename in
    Out_channel.with_open_text (build_file_name sheet lytexfilename) (fun t ->
        Out_channel.output_string t data);

    (* run lilypond on .ly *)
    let command =
      sprintf
        "cp %s/%s %s/. && (cd %s  && lilypond %s || true ) &&  (cd %s  && \
         lilypond %s ) "
        sheet.Sheet.srcdir filename sheet.Sheet.tmpdir
        (* cd *) sheet.Sheet.tmpdir filename (* cd *) sheet.Sheet.tmpdir
        filename
    in
    Command.run ~message:"lilypond" ~command;

    (* run lilypond-book on .lytex *)
    let command =
      sprintf
        "(cd %s  && lilypond-book --pdf %s || true ) &&  (cd %s  && \
         lilypond-book --pdf %s ) "
        (* cd *) sheet.Sheet.tmpdir lytexfilename (* cd *) sheet.Sheet.tmpdir
        lytexfilename
    in
    Command.run ~message:"lilypond-book" ~command
  in
  List.iter ~f:(fun filename -> run ~filename) sheet.Sheet.lilypondfiles;
  ()
