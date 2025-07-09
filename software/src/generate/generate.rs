use crate::generate::handlebars_helpers::get_handlebar;
use crate::model::model::{Book, Song, StructureItemContent, World};
use std::fs::File;
use std::io::{Error, Write};
// use std::os::unix::fs::PermissionsExt;
use crate::helpers::duration::duration_of_song;
use crate::helpers::helpers::song_of_booksong;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::helpers::io::{copy_file, create_dir_all, write};

pub fn mount_files(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut p: PathBuf = world.builddir.clone();
        p.push("delivery");
        let _ = create_dir_all(&p)?;
    }

    {
        // mount files
        for song in &world.songs {
            {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push("lyrics");
                let _ = create_dir_all(&pfrom)?;
                let mut pto = PathBuf::from(song.builddir.clone());
                pto.push("lyrics");
                let _ = create_dir_all(&pto)?;
            }
            {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push(format!("body.tex"));
                if !pfrom.exists() {
                    return Err(
                        format!("no body.tex for song {}/{}", song.author, song.title).into(),
                    );
                }
                let mut pto = PathBuf::from(song.builddir.clone());
                pto.push(format!("body.tex"));
                fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
            }
            {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push(format!("add.tikz"));
                if !pfrom.exists() {
                    return Err(
                        format!("no add.tikz for song {}/{}", song.author, song.title).into(),
                    );
                }
                let mut pto = PathBuf::from(song.builddir.clone());
                pto.push(format!("add.tikz"));
                fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
            }

            {
                for lyfile in &song.lilypondfiles {
                    let mut pfrom = PathBuf::from(song.srcdir.clone());

                    pfrom.push(&lyfile);
                    if !pfrom.exists() {
                        return Err(format!(
                            "no {} for song {}/{}",
                            lyfile, song.author, song.title
                        )
                        .into());
                    }
                    let mut pto = PathBuf::from(song.builddir.clone());
                    pto.push(lyfile);
                    fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
                }
            }

            let lyrics_ids = &song
                .structure
                .iter()
                .filter_map(|item| match &item.item {
                    StructureItemContent::ItemChords(s) => Some(s.section_id.clone()),
                    StructureItemContent::ItemRef(s) => Some(s.section_id.clone()),
                    StructureItemContent::ItemHRule(_) => None,
                    StructureItemContent::ItemNewColumn => None,
                })
                .collect::<Vec<_>>();
            for id in lyrics_ids {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push("lyrics");
                pfrom.push(format!("{}.tex", id));
                if !pfrom.exists() {
                    std::fs::write(pfrom.to_str().unwrap(), "\\color{red}{put something here}")?;
                }
                let mut pto = PathBuf::from(song.builddir.clone());
                pto.push("lyrics");
                pto.push(format!("{}.tex", id));
                fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
            }
        }
    }
    Ok(())
}

pub fn generate_from_handlebars_templates(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    // {
    //     let bytes = include_bytes!("../../others/shfiles/make_lytex.sh");
    //     let mut p: PathBuf = world.builddir.clone();
    //     let () = create_dir_all(&p)?;
    //     p.push("make_lytex.sh");
    //     log::debug!("write {}", p.display());
    //     let _ = write(&p, bytes)?;
    // }
    {
        let bytes = include_bytes!("../../others/shfiles/colors.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("colors.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/add-missing-lyrics.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("add-missing-lyrics.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/check-existence-of-files.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("check-existence-of-files.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_clean.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("make_clean.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/check-json.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("check-json.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_mpost.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("make_mpost.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/rkill.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("rkill.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/pdf_song_script.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("pdf_song_script.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/pdf_book_script.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("pdf_book_script.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/lytex_script.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("lytex_script.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/md5sum.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("md5sum.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/delivery_script.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("delivery_script.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_gdrive.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("make_gdrive.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/preamble.tex");
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            p.push("preamble.tex");
            let _ = write(&p, bytes_preamble_tex)?;
        }
        for book in &world.books {
            let mut p: PathBuf = book.builddir.clone();
            p.push("preamble.tex");
            let _ = write(&p, bytes_preamble_tex)?;
        }
    }

    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/main.tex");
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            p.push("main.tex");
            let _ = write(&p, bytes_preamble_tex)?;
        }
    }
    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/tikzlibraryspline.code.tex");
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            p.push("tikzlibraryspline.code.tex");
            let _ = write(&p, bytes_preamble_tex)?;
        }
        for books in &world.books {
            let mut p: PathBuf = books.builddir.clone();
            p.push("tikzlibraryspline.code.tex");
            let _ = write(&p, bytes_preamble_tex)?;
        }
    }

    {
        let bytes_chords_tex = include_bytes!("../../others/texfiles/chords.tex");
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            p.push("chords.tex");
            let _ = write(&p, bytes_chords_tex)?;
        }
        for book in &world.books {
            let mut p: PathBuf = book.builddir.clone();
            p.push("chords.tex");
            let _ = write(&p, bytes_chords_tex)?;
        }
    }
    {
        let template =
            String::from_utf8(include_bytes!("../../others/lyfiles/macros.ly").to_vec())?;
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            let _ = create_dir_all(&p)?;
            p.push("macros.ly");
            let mut h = get_handlebar()?;
            h.register_template_string("t1", &template)?;
            // let sections: Vec<Section> = world.sections.iter().map(|x| x.1.clone()).collect();
            // let j = handlebars::to_json(&sections);
            let output_data = h.render("t1", song)?;
            let mut output = File::create(p)?;
            let _ = output.write(output_data.as_bytes())?;
        }
    }

    {
        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/sections.tex").to_vec())?;
        let mut h = get_handlebar()?;
        h.register_template_string("t1", template)?;
        let output_data = h.render("t1", world)?;
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            p.push("sections.tex");
            let mut output = File::create(p)?;
            let _ = output.write(output_data.as_bytes())?;
        }
        for book in &world.books {
            let mut p: PathBuf = book.builddir.clone();
            p.push("sections.tex");
            let mut output = File::create(p)?;
            let _ = output.write(output_data.as_bytes())?;
        }
    }

    {
        for song in &world.songs {
            // {
            //     let mut output = File::create("debug.json")?;
            //     write!(output, "{}", serde_json::to_string(&song)?)?;
            // }
            let mut p: PathBuf = song.builddir.clone();
            create_dir_all(&p)?;
            p.push("data.tex");
            log::debug!("write {}", p.display());
            let mut output = File::create(&p)?;
            writeln!(output, "% length of structure : {}", song.structure.len())?;

            let template =
                String::from_utf8(include_bytes!("../../others/texfiles/data.tex").to_vec())?;

            let mut h = get_handlebar()?;
            h.register_template_string("t1", template)?;
            let output_data = h.render("t1", song)?;
            let _ = output.write(output_data.as_bytes())?;
        }
    }

    {
        // song.tikz
        let mut id = 0u32;
        for song in &world.songs {
            id += 1;
            let mut p: PathBuf = song.builddir.clone();
            let _ = create_dir_all(&p)?;
            p.push("song.tikz");
            let mut output = File::create(&p)?;

            let songtikzuserinput = {
                let mut p: PathBuf = song.builddir.clone();
                p.push("add.tikz");
                fs::read_to_string(p)?
            };

            let template =
                String::from_utf8(include_bytes!("../../others/texfiles/song.tikz").to_vec())?;

            #[derive(serde::Serialize)]
            struct XXX<'a> {
                song: &'a Song,
                songtikzuserinput: String,
                id: u32,
            }
            let data = XXX {
                song,
                songtikzuserinput,
                id,
            };
            log::debug!("generate song.tikz");
            let mut h = get_handlebar()?;
            h.register_template_string("t1", template)?;
            let output_data = h.render("t1", &data)?;
            let _ = output.write_all(output_data.as_bytes())?;
        }
    }

    {
        // setlist.tikz
        let mut id = 0u32;
        for book in &world.books {
            id = id + 1;
            let mut p: PathBuf = book.builddir.clone();
            let _ = create_dir_all(&p)?;
            p.push("book-setlist.tikz");
            let mut output = File::create(&p)?;

            let template = String::from_utf8(
                include_bytes!("../../others/texfiles/book-setlist.tikz").to_vec(),
            )?;

            #[derive(serde::Serialize)]
            struct SSS {
                author: String,
                title: String,
                duration: String,
                cumul_duration: String,
                tempo: u32,
            }
            #[derive(serde::Serialize)]
            struct XXX {
                songs: Vec<SSS>,
            }
            let mut cumul = chrono::Duration::new(0, 0).unwrap();
            let songs = book
                .songs
                .iter()
                .map(|bs| {
                    let song = song_of_booksong(&world, &bs).unwrap();
                    let d = duration_of_song(&song);
                    let minutes = d.num_minutes();
                    let seconds = d.num_seconds() - 60 * minutes;
                    cumul += d;
                    let cumul_minutes = cumul.num_minutes();
                    let cumul_seconds = cumul.num_seconds() - 60 * cumul_minutes;
                    SSS {
                        author: song.author.clone(),
                        title: song.title.to_string(),
                        duration: format!("{:2}'{:02}\"", minutes, seconds),
                        cumul_duration: format!("{:2}'{:02}\"", cumul_minutes, cumul_seconds),
                        tempo: song.tempo,
                    }
                })
                .collect::<Vec<_>>();
            let data = XXX { songs };
            log::debug!("generate book-setlist.tikz");
            let mut h = get_handlebar()?;
            h.register_template_string("t1", template)?;
            let output_data = h.render("t1", &data)?;
            output.write_all(output_data.as_bytes())?;
        }
    }

    Ok(())
}

pub fn generate_json_song(song: &Song) -> Result<(), Error> {
    log::debug!("generate song.json in {}", song.builddir.display());
    let mut p: PathBuf = song.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("song-internal.json");
    let _ = fs::write(
        p.to_str().unwrap(),
        serde_json::to_string(&song).unwrap().as_bytes(),
    );

    Ok(())
}

pub fn generate_json_book(book: &Book) -> Result<(), Error> {
    log::debug!("generate book.json in {}", book.builddir.display());
    let mut p: PathBuf = book.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("book.json");
    let _ = fs::write(
        p.to_str().unwrap(),
        serde_json::to_string(&book).unwrap().as_bytes(),
    );

    Ok(())
}

pub fn generate_main_book(book: &Book) -> Result<(), Error> {
    log::debug!("generate main.tex in {}", book.builddir.display());
    let mut p: PathBuf = book.builddir.clone();
    let _ = fs::create_dir_all(&p)?;
    p.push("main.tex");
    let mut output = File::create(p)?;
    let template =
        String::from_utf8(include_bytes!("../../others/texfiles/mainbook.tex").to_vec()).unwrap();

    let mut h = get_handlebar()?;
    h.register_template_string("t1", template).unwrap();
    let output_data = h.render("t1", book).unwrap();
    let _ = output.write(output_data.as_bytes()).unwrap();

    Ok(())
}

pub fn generate_for_aws_lambda(builddir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    {
        let pfrom = PathBuf::from("/root/.fonts");
        if !pfrom.exists() {
            return Err("/root/.fonts does not exist".into());
        }
        if !pfrom.is_dir() {
            return Err("/root/.fonts is not a directory".into());
        }
        log::info!("now looking for fonts");
        let mut pto = builddir.clone();
        pto.push(".fonts");
        let _ = fs::create_dir_all(&pto)?;
        for pfont in WalkDir::new(pfrom).into_iter().filter_map(|e| e.ok()) {
            let pfont = pfont.path().to_path_buf();
            log::info!("font : {:?}", pfont);

            if pfont.is_file() && pfont.extension() == Some(OsStr::new("ttf")) {
                let mut ptofont = pto.clone();
                ptofont.push(pfont.file_name().ok_or("huh...")?);
                log::info!("ptofont : {:?}", &ptofont);

                copy_file(&PathBuf::from(pfont), &ptofont)?;
            }
        }
    }
    // let mut p = builddir.clone();
    // p.push(".texlive2021");
    // create_dir_all(&p)?;
    log::info!("create /mnt/efs/zik/build/.texlive2021/texmf-var/web2c");
    let mut p = PathBuf::from("/mnt/efs/zik/build/.texlive2021/texmf-var/web2c");
    create_dir_all(&p)?;

    Ok(())
}
