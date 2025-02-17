use crate::model::model::Book;

pub fn pdfname_of_book(book: &Book) -> String {
    let pdfname = &book.title;
    let pdfname = normalize_name(pdfname.to_owned());
    pdfname
}

pub fn normalize_pdf_name(author: &String, title: &String) -> String {
    normalize_name(format!("{author}--@--{title}", author = author, title = title).clone())
}

pub fn normalize_name(input: String) -> String {
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
