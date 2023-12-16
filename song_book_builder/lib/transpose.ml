module Log = Dolog.Log

type flat_or_sharp_t = NoSF | Sharp | Flat

let flat_or_sharp_of_chord chord =
  let ret =
    match String.length chord with
    | 0 -> failwith "huh ?"
    | 1 -> NoSF
    | _ -> (
        match String.get chord 1 with '#' -> Sharp | 'b' -> Flat | _ -> NoSF)
  in
  ret

let rest_of_chord chord =
  let start =
    match String.length chord with
    | 0 -> failwith "huh ?"
    | 1 -> 1
    | _ -> ( match String.get chord 1 with '#' -> 2 | 'b' -> 2 | _ -> 1)
  in
  String.sub chord start (String.length chord - start)

let semitone_of_chord chord =
  let c = String.get chord 0 in
  let h0 = Uchar.to_int (Uchar.of_char 'A') in
  let h1 = Uchar.to_int (Uchar.of_char c) - h0 in
  let h2 =
    match h1 with
    | 0 -> 0
    | 1 -> 2
    | 2 -> 3
    | 3 -> 5
    | 4 -> 7
    | 5 -> 8
    | 6 -> 10
    | 7 -> 12
    | _ -> failwith "bad transpose"
  in
  (*  let _ = Log.info "%s -> %d -> %d" chord h1 transpose in *)
  let h2 =
    match flat_or_sharp_of_chord chord with
    | NoSF -> h2
    | Sharp -> h2 + 1
    | Flat -> h2 - 1
  in
  (*  let _ = Log.info "%s -> %d -> %d" chord h1 h2 in *)
  h2

let transpose_chord ~chord ~transpose =
  (*  let h = semitone_of_chord chord in *)
  (*  let new_h = match h with None i | Sharp i | Flat i -> transpose + i in *)
  (*  let _ = new_h in *)
  (*  *)
  let _ = chord in
  let _ = transpose in
  let _from, _to = match transpose with None -> ("A", "A") | Some p -> p in
  let flat_or_sharp = flat_or_sharp_of_chord _to in
  let offset = semitone_of_chord _to - semitone_of_chord _from in
  let _ = Log.info "offset is %d" offset in
  let _ = Log.info "offset is %d" offset in
  let _ = Log.info "xxxx is %s %d" chord (semitone_of_chord chord) in
  let new_chord_semitone = semitone_of_chord chord + offset in
  let new_chord_semitone =
    if new_chord_semitone < 0 then new_chord_semitone + 12
    else new_chord_semitone
  in
  let new_chord_semitone =
    if new_chord_semitone > 11 then new_chord_semitone - 12
    else new_chord_semitone
  in
  let _ = Log.info "xxxx is %s %d" chord (semitone_of_chord chord) in

  let new_chord =
    match (new_chord_semitone, flat_or_sharp) with
    | 0, _ -> "A"
    | 1, Flat -> "Bb"
    | 1, Sharp -> "A#"
    | 2, _ -> "B"
    | 3, _ -> "C"
    | 4, NoSF -> "C#"
    | 4, Flat -> "Db"
    | 4, Sharp -> "C#"
    | 5, _ -> "D"
    | 6, Flat -> "Eb"
    | 6, Sharp -> "D#"
    | 7, _ -> "E"
    | 8, _ -> "F"
    | 9, Flat -> "Gb"
    | 9, NoSF -> "F#"
    | 9, Sharp -> "F#"
    | 10, _ -> "G"
    | 11, NoSF -> "G#"
    | 11, Flat -> "Ab"
    | 11, Sharp -> "G#"
    | _, NoSF -> failwith (string_of_int new_chord_semitone) ^ " ; NoSF"
    | _, Flat -> failwith (string_of_int new_chord_semitone) ^ " ; Sharp"
    | _, Sharp -> failwith (string_of_int new_chord_semitone) ^ " ; Flat"
  in
  let _ = Log.info "offset : %d" offset in
  new_chord ^ rest_of_chord chord
