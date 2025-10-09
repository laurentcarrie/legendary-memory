use crate::actions::mount::mount_from_data;
use crate::generate::handlebars_helpers::get_handlebar;
use crate::helpers::duration::duration_of_song;
use crate::helpers::helpers::song_of_booksong;
use crate::helpers::io::{create_dir_all, write_string};
use crate::helpers::path::make_path;
use crate::model::use_model as M;
// use std::path::PathBuf;

fn make_one_page_toc(
    world: M::World,
    offset: usize,
    number_of_songs_in_one_toc_page: usize,
    book: M::Book,
    mut cumul: chrono::Duration,
) -> Result<u32, Box<dyn std::error::Error>> {
    let template =
        String::from_utf8(include_bytes!("../../others/texfiles/book-setlist.tikz").to_vec())?;

    #[derive(serde::Serialize, Clone)]
    struct Sss {
        author: String,
        title: String,
        duration: String,
        cumul_duration: String,
        tempo: u32,
    }
    #[derive(serde::Serialize, Clone)]
    struct Xxx {
        songs: Vec<Sss>,
        offset: usize,
    }
    // let mut cumul = cumul ;
    let songs = book
        .songs
        .iter()
        .map(|bs| {
            let song = song_of_booksong(&world, bs).unwrap();
            let d = duration_of_song(&song);
            let minutes = d.num_minutes();
            let seconds = d.num_seconds() - 60 * minutes;
            cumul += d;
            let cumul_minutes = cumul.num_minutes();
            let cumul_seconds = cumul.num_seconds() - 60 * cumul_minutes;
            Sss {
                author: song.author.clone(),
                title: song.title.to_string(),
                duration: format!("{minutes:2}'{seconds:02}\""),
                cumul_duration: format!("{cumul_minutes:2}'{cumul_seconds:02}\""),
                tempo: song.tempo,
            }
        })
        .collect::<Vec<_>>();
    let songs = songs[offset..offset + number_of_songs_in_one_toc_page].to_vec();
    let data = Xxx { songs, offset };
    log::debug!("generate book-setlist-{offset}.tikz");
    let mut h = get_handlebar()?;
    h.register_template_string("t1", template)?;
    let output_data = h.render("t1", &data)?;
    let changed = mount_from_data(
        output_data.into(),
        make_path(
            book.builddir.clone(),
            vec![format!("book-setlist-{offset}.tikz").as_str()],
        ),
    )?;

    Ok(changed)
}

// fn make_maintex_book(
//     _world: M::World,
//     book: M::Book,
//     number_of_songs_in_one_toc_page: usize,
// ) -> Result<u32, Box<dyn std::error::Error>> {
//     let ret = _make_maintex_book(_world, book, number_of_songs_in_one_toc_page, false)?;
//     Ok(ret)
// }

fn make_maintex_book(
    _world: M::World,
    book: M::Book,
    number_of_songs_in_one_toc_page: usize,
) -> Result<u32, Box<dyn std::error::Error>> {
    let template =
        String::from_utf8(include_bytes!("../../others/texfiles/mainbook.tex").to_vec())?;
    #[derive(serde::Serialize, Clone)]
    struct Sss {
        songs: Vec<M::BookSong>,
        offsets: Vec<String>,
        lyrics_only: bool,
        title: String,
        cover_image: bool,
    }

    let mut offsets: Vec<String> = Vec::new();
    let mut current_offset = 0;

    while current_offset < book.songs.len() {
        offsets.push(format!("book-setlist-{current_offset}.tikz"));
        current_offset += number_of_songs_in_one_toc_page;
    }
    let data = Sss {
        songs: book.songs,
        offsets,
        lyrics_only: book.lyrics_only,
        title: book.title,
        cover_image: book.cover_image,
    };
    let mut h = get_handlebar()?;
    h.register_template_string("t1", template)?;
    let output_data = h.render("t1", &data)?;
    let changed = mount_from_data(
        output_data.into(),
        make_path(book.builddir.clone(), vec!["main.tex"]),
    )?;

    Ok(changed)
}

pub async fn run(
    world: M::World,
    book: M::Book,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut target = book.builddir.clone();
    target.push(format!("bootstrap"));
    create_dir_all(&book.builddir)?;

    let mut changed = 0;

    changed += mount_from_data(
        include_bytes!("../../others/texfiles/preamble.tex").to_vec(),
        make_path(book.builddir.clone(), vec!["preamble.tex"]),
    )?;

    changed += mount_from_data(
        include_bytes!("../../others/texfiles/tikzlibraryspline.code.tex").to_vec(),
        make_path(book.builddir.clone(), vec!["tikzlibraryspline.code.tex"]),
    )?;

    changed += mount_from_data(
        include_bytes!("../../others/texfiles/chords.tex").to_vec(),
        make_path(book.builddir.clone(), vec!["chords.tex"]),
    )?;

    {
        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/sections.tex").to_vec())?;
        let mut h = get_handlebar()?;
        h.register_template_string("t1", template)?;
        let output_data = h.render("t1", &world)?;
        changed += mount_from_data(
            output_data.into(),
            make_path(book.builddir.clone(), vec!["sections.tex"]),
        )?;
    }

    // number of songs in one toc page
    let number_of_songs_in_one_toc_page: usize = 12;

    {
        let mut tikznames = Vec::new();
        let mut offset = 0;
        let cumul = chrono::Duration::new(0, 0).expect("null duration");
        while offset < book.songs.len() {
            let l = if offset + number_of_songs_in_one_toc_page > book.songs.len() {
                book.songs.len() - offset
            } else {
                number_of_songs_in_one_toc_page
            };
            tikznames.push(format!(
                "book-setlist-{}.tikz",
                offset / number_of_songs_in_one_toc_page
            ));
            make_one_page_toc(world.clone(), offset, l, book.clone(), cumul)?;
            offset += number_of_songs_in_one_toc_page;
        }
    }

    changed += make_maintex_book(world.clone(), book.clone(), number_of_songs_in_one_toc_page)?;

    write_string(&target, &"".to_string())?;

    if changed > 0 {
        Ok(M::BuildType::Rebuilt(target))
    } else {
        Ok(M::BuildType::NotTouched(target))
    }
}
