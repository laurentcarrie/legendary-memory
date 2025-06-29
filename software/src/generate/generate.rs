use crate::generate::handlebars_helpers::get_handlebar;
use crate::model::use_model::{Book, Song, StructureItemContent, World};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::helpers::io::{copy_file, create_dir_all};

pub fn mount_files(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut p: PathBuf = world.builddir.clone();
        p.push("delivery");
        create_dir_all(&p)?;
    }

    {
        // mount files
        for song in &world.songs {
            {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push("lyrics");
                create_dir_all(&pfrom)?;
                let mut pto = song.builddir.clone();
                pto.push("lyrics");
                create_dir_all(&pto)?;
            }
            {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push("body.tex");
                if !pfrom.exists() {
                    return Err(
                        format!("no body.tex for song {}/{}", song.author, song.title).into(),
                    );
                }
                let mut pto = song.builddir.clone();
                pto.push("body.tex");
                fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
            }
            {
                let mut pfrom = PathBuf::from(song.srcdir.clone());
                pfrom.push("add.tikz");
                if !pfrom.exists() {
                    return Err(
                        format!("no add.tikz for song {}/{}", song.author, song.title).into(),
                    );
                }
                let mut pto = song.builddir.clone();
                pto.push("add.tikz");
                fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
            }

            {
                for lyfile in &song.lilypondfiles {
                    let mut pfrom = PathBuf::from(song.srcdir.clone());

                    pfrom.push(lyfile);
                    if !pfrom.exists() {
                        return Err(format!(
                            "no {} for song {}/{}",
                            lyfile, song.author, song.title
                        )
                        .into());
                    }
                    let mut pto = song.builddir.clone();
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
                pfrom.push(format!("{id}.tex"));
                if !pfrom.exists() {
                    std::fs::write(pfrom.to_str().unwrap(), "\\color{red}{put something here}")?;
                }
                let mut pto = song.builddir.clone();
                pto.push("lyrics");
                pto.push(format!("{id}.tex"));
                fs::copy(pfrom.to_str().unwrap(), pto.to_str().unwrap())?;
            }
        }
    }
    Ok(())
}

pub fn generate_json_song(song: &Song) -> Result<(), Error> {
    log::debug!("generate song.json in {}", song.builddir.display());
    let mut p: PathBuf = song.builddir.clone();
    fs::create_dir_all(&p)?;
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
    fs::create_dir_all(&p)?;
    p.push("book.json");
    fs::write(
        p.to_str().unwrap(),
        serde_json::to_string(&book).unwrap().as_bytes(),
    )?;

    Ok(())
}

// pub fn generate_main_book(book: &Book) -> Result<(), Error> {
//     log::debug!("generate main.tex in {}", book.builddir.display());
//     let mut p: PathBuf = book.builddir.clone();
//     fs::create_dir_all(&p)?;
//     p.push("main.tex");
//     let mut output = File::create(p)?;
//     let template =
//         String::from_utf8(include_bytes!("../../others/texfiles/mainbook.tex").to_vec()).unwrap();

//     let mut h = get_handlebar()?;
//     h.register_template_string("t1", template).unwrap();
//     let output_data = h.render("t1", book).unwrap();
//     output.write_all(output_data.as_bytes())?;

//     Ok(())
// }

pub fn generate_for_aws_lambda(builddir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    {
        let pfrom = PathBuf::from("/var/task/.fonts");
        if !pfrom.exists() {
            return Err(format!("{} does not exist", pfrom.display()).into());
        }
        if !pfrom.is_dir() {
            return Err(format!("{} is not a directory", pfrom.display()).into());
        }
        log::info!("now looking for fonts");
        let mut pto = builddir.to_path_buf();
        pto.push(".fonts");
        fs::create_dir_all(&pto)?;
        for pfont in WalkDir::new(pfrom).into_iter().filter_map(|e| e.ok()) {
            let pfont = pfont.path().to_path_buf();

            if pfont.is_file() && pfont.extension() == Some(OsStr::new("ttf")) {
                let mut ptofont = pto.clone();
                ptofont.push(pfont.file_name().ok_or("huh...")?);

                copy_file(&pfont, &ptofont)?;
            }
        }
    }
    // let mut p = builddir.clone();
    // p.push(".texlive2021");
    // create_dir_all(&p)?;
    log::info!("create {}/.texlive2021/texmf-var/web2c", builddir.display());
    let mut p = builddir.to_path_buf();
    p.push(".texlive2021/texmf-var/web2c");
    create_dir_all(&p)?;
    let mut perms = fs::metadata(&p)?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(p, perms)?;

    Ok(())
}
