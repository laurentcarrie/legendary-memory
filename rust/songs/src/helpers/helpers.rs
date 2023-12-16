use crate::config::model::Song;

pub fn pdfname_of_song(song: &Song) -> String {
    let pdfname = &song.author;
    let pdfname = normalize_name(&(pdfname.to_owned() + &"--@--".to_string() + &song.title));
    pdfname
}

pub fn normalize_name(input: &String) -> String {
    let mut output = input.clone();
    output.make_ascii_lowercase();
    output = output
        .replace(" ", "_")
        .replace("/", "_")
        .replace(".", "_")
        .replace(")", "_")
        .replace("(", "_")
        .replace("'", "_");
    output
}
