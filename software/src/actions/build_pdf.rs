use crate::actions::build_lytex::build_lytex;

use crate::helpers::helpers::song_of_booksong;
use crate::helpers::io::{copy_file, read_to_string, read_to_vec_u8, write_string};
use crate::model::model::{
    Book, ELogType, LogItem, LogItemBook, LogItemSong, Song, StructureItemContent, World,
};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::mpsc::Sender;

fn needs_rerun(path: PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let mut p = path.clone();
    p.push("main.log");
    let data = read_to_string(&p)?;
    let re = Regex::new(r".*Rerun to get.*").unwrap();
    let len = re.find_iter(data.as_str()).collect::<Vec<_>>().len();
    Ok(len > 0)
}
/// compute the digest of the sources of a song
/// song.json ( for the structure )
/// the lyrics, the additional tex files, the lilypond files
fn compute_digest(song: &Song) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&song)?.as_bytes());

    let mut lyrics_files = song
        .structure
        .iter()
        .filter_map(|section| match &section.item {
            StructureItemContent::ItemChords(chords) => Some(chords.section_id.clone()),
            StructureItemContent::ItemHRule(_) => None,
            StructureItemContent::ItemNewColumn => None,
            StructureItemContent::ItemRef(xref) => Some(xref.section_id.clone()),
        })
        .map(|id| {
            let mut p = PathBuf::from(&song.srcdir.clone());
            p.push("lyrics");
            p.push(format!("{id}.tex"));
            p
        })
        .collect::<Vec<_>>();

    let mut texfiles = song
        .texfiles
        .iter()
        .map(|f| {
            let mut p = PathBuf::from(&song.srcdir.clone());
            p.push(f);
            p
        })
        .collect::<Vec<_>>();

    let mut lyfiles = song
        .lilypondfiles
        .iter()
        .map(|f| {
            let mut p = PathBuf::from(&song.srcdir.clone());
            p.push(f);
            p
        })
        .collect::<Vec<_>>();

    let mut all_files: Vec<PathBuf> = vec![];
    all_files.append(&mut lyrics_files);
    for f in vec!["song.json", "body.tex", "add.tikz"] {
        all_files.push({
            let mut p = PathBuf::from(&song.srcdir.clone());
            p.push(f);
            p
        });
    }
    all_files.append(&mut texfiles);
    all_files.append(&mut lyfiles);

    for p in all_files {
        let contents = read_to_string(&p)?;
        hasher.update(contents);
    }
    let result = hasher.finalize();
    let ret = hex::encode(result.to_vec());
    Ok(ret)
}

// computes the digest of the sources PLUS the output pdf file
// Error if the pdf file does not exist
pub fn compute_digest_ok(world: &World, song: &Song) -> Result<String, Box<dyn std::error::Error>> {
    // log::info!("{}:{}", file!(), line!());
    let digest = compute_digest(song)?;
    let mut hasher = Sha256::new();
    // log::info!("{}:{}", file!(), line!());
    hasher.update(&digest);
    // log::info!("{}:{}", file!(), line!());
    {
        let mut p = PathBuf::from(&world.builddir);
        p.push("delivery");
        // log::info!("{}:{}", file!(), line!());
        p.push(format!("{}.pdf", &song.pdfname));
        // log::info!("{}:{} {:?}", file!(), line!(), &p);
        let contents = read_to_vec_u8(&p)?;
        // log::info!("{}:{}", file!(), line!());
        hasher.update(contents);
    }
    // log::info!("{}:{}", file!(), line!());
    let result = hasher.finalize();
    // log::info!("{}:{}", file!(), line!());
    let data = hex::encode(&result[..].to_vec());
    // log::info!("{}:{} {}", file!(), line!(),&data);
    Ok(data)
}

fn pathbuf_ok_checksum(song: &Song) -> PathBuf {
    let mut p = PathBuf::from(&song.builddir);
    p.push(".checksum_ok");
    p
}
// fn pathbuf_failed_checksum(song: &Song) -> PathBuf {
//     let mut p = PathBuf::from(&song.builddir);
//     p.push(".checksum_failed");
//     p
// }

fn __vec_are_equal<T: std::cmp::PartialEq>(v1: Vec<T>, v2: Vec<T>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for x in v1.iter().zip(v2.iter()) {
        if x.0 != x.1 {
            return false;
        }
    }
    true
}

/// true : at least one song needs rebuild
fn needs_rebuild_book_ok(world: &World, book: &Book) -> Result<bool, Box<dyn std::error::Error>> {
    for bs in &book.songs {
        let song = song_of_booksong(world, bs)?;
        if needs_rebuild_ok(&world, &song) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// true : song needs rebuild
fn needs_rebuild_ok(world: &World, song: &Song) -> bool {
    let digest = read_to_string(&pathbuf_ok_checksum(&song));
    match digest {
        Err(_) => true,
        Ok(v1) => match compute_digest_ok(&world, &song) {
            Ok(v2) => v1 != v2,
            Err(_) => true,
        },
    }
}

// fn needs_rebuild_failed(song: &Song) -> Result<bool, Box<dyn std::error::Error>>  {
//     let digest = read_to_string(&pathbuf_failed_checksum(&song));
//     match digest {
//         Err(_) => Ok(false),
//         Ok(data) => {
//             let v1 = data.as_bytes().to_vec();
//             let v2 = compute_digest(&song)?;
//             return Ok(vec_are_equal(v1, v2));
//         }
//     }
// }

pub async fn wrapped_build_pdf_song(
    tx: Sender<LogItem>,
    world: World,
    song: Song,
    force_rebuild: bool,
) -> () {
    match build_pdf_song(tx, world.clone(), song.clone(), force_rebuild).await {
        Ok(()) => (),
        Err(e) => log::error!(
            "wrapped build pdf, {} {} ; {}",
            &song.author,
            &song.title,
            e.to_string()
        ),
    }
}

pub async fn wrapped_build_pdf_book(tx: Sender<LogItem>, world: World, book: Book) -> () {
    match build_pdf_book(tx, world.clone(), book.clone()).await {
        Ok(()) => (),
        Err(e) => log::error!("wrapped build pdf, {} ; {}", &book.title, e.to_string()),
    }
}

pub async fn build_pdf_song(
    tx: Sender<LogItem>,
    world: World,
    song: Song,
    force_rebuild: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("build pdf {} {}", &song.author, &song.title);
    let p = pathbuf_ok_checksum(&song);
    if force_rebuild {
        if p.exists() {
            std::fs::remove_file(p.as_path())?;
        }
    }
    if !needs_rebuild_ok(&world, &song) {
        log::info!("does not need rebuild {} {}", &song.author, &song.title);
        let li = LogItem::Song(LogItemSong {
            author: song.author.clone(),
            title: song.title.clone(),
            status: ELogType::NoNeedSuccess,
        });
        tx.send(li).await?;
        return Ok(());
    }

    // if needs_rebuild_failed(&song)? {
    //     let li = LogItem::Song(LogItemSong {
    //         author: song.author.clone(),
    //         title: song.title.clone(),
    //         status: ELogType::NoNeedFailed,
    //     });
    //     tx.send(li).await.unwrap();
    //     return Ok(());
    // }

    let li = LogItem::Song(LogItemSong {
        author: song.author.clone(),
        title: song.title.clone(),
        status: ELogType::Started,
    });
    tx.send(li).await?;

    let mut success: bool = true;

    for lyfile in &song.lilypondfiles {
        let li = LogItem::Song(LogItemSong {
            author: song.author.clone(),
            title: song.title.clone(),
            status: ELogType::Lilypond(lyfile.clone()),
        });
        tx.send(li).await?;
        match build_lytex(song.clone(), lyfile.clone()).await {
            Ok(()) => (),
            Err(_) => success = false,
        }
    }

    let mut count = 1;
    loop {
        let fout = File::create(PathBuf::from("lualatex.log"))?;
        if !success {
            break;
        };
        let li = LogItem::Song(LogItemSong {
            author: song.author.clone(),
            title: song.title.clone(),
            status: ELogType::Lualatex(count as u32),
        });
        tx.send(li).await?;
        let child = Command::new("lualatex")
            .arg("--interaction=nonstopmode")
            .arg("main.tex")
            .kill_on_drop(true)
            // .stdout(Stdio::piped())
            .stdout(fout)
            .current_dir(&song.builddir)
            .spawn()?;

        log::info!(
            "{}:{} run lualatex for {} {}",
            file!(),
            line!(),
            &song.author,
            &song.title
        );
        let output = &child.wait_with_output().await?;

        if !(output.status.success()) {
            success = false;
            log::error!("lualatex failed for {} {}", &song.author, &song.title);
            break;
        }
        if !needs_rerun(song.builddir.clone())? {
            success = true;
            break;
        }
        count = count + 1;
    }

    {
        let mut pfrom = PathBuf::from(&song.builddir);
        pfrom.push("main.pdf");
        let mut pto = PathBuf::from(&song.builddir);
        pto.push(format!("{}.pdf", &song.pdfname));
        copy_file(&pfrom, &pto)?;

        let mut pto = PathBuf::from(&world.builddir);
        pto.push("delivery");
        pto.push(format!("{}.pdf", &song.pdfname));
        // copy_file(&pfrom, &pto)?;

        let fout = File::create(PathBuf::from("lualatex.log"))?;
        let li = LogItem::Song(LogItemSong {
            author: song.author.clone(),
            title: song.title.clone(),
            status: ELogType::Ps2pdf,
        });
        tx.send(li).await?;
        let child = Command::new("ps2pdf")
            .arg("main.pdf")
            .arg(pto.to_str().unwrap())
            .kill_on_drop(true)
            // .stdout(Stdio::piped())
            .stdout(fout)
            .current_dir(&song.builddir)
            .spawn()?;

        log::info!(
            "{}:{} run ps2pdf for {} {}",
            file!(),
            line!(),
            &song.author,
            &song.title
        );
        // log::info!("{}:{} {:?}", file!(), line!(), &child);
        let output = &child.wait_with_output().await?;
        if !(output.status.success()) {
            success = false;
            log::error!("ps2pdf failed for {} {}", &song.author, &song.title);
        }
    }

    if !success {
        let li = LogItem::Song(LogItemSong {
            author: song.author.clone(),
            title: song.title.clone(),
            status: ELogType::Failed,
        });
        log::info!(
            "tx.send final error result for {} {}",
            &song.author,
            &song.title
        );
        tx.send(li).await?;
        return Ok(());
    }

    if success {
        // write ok checksum
        let checksum = compute_digest_ok(&world, &song)?;
        let _x = write_string(&pathbuf_ok_checksum(&song), &checksum)?;
        // assert!(needs_rebuild_ok(&song) == false);
    }

    let li = LogItem::Song(LogItemSong {
        author: song.author.clone(),
        title: song.title.clone(),
        status: if success {
            ELogType::Success
        } else {
            ELogType::Failed
        },
    });
    log::info!(
        "tx.send final result for song {}  {} ",
        &song.author,
        &song.title
    );
    tx.send(li).await?;
    Ok(())
}

async fn build_pdf_book(
    tx: Sender<LogItem>,
    world: World,
    book: Book,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("build book {}", &book.title);
    loop {
        if !needs_rebuild_book_ok(&world, &book)? {
            break;
        }
        log::info!(
            "wait for building book {} ... some songs are not done yet",
            book.title
        );
        let _x = tokio::time::sleep(Duration::from_secs(5)).await;
    }
    let li = LogItem::Book(LogItemBook {
        title: book.title.clone(),
        status: ELogType::Started,
    });
    tx.send(li).await?;

    let mut success: bool = true;

    let mut count = 1;
    loop {
        if !success {
            break;
        };
        let fout = File::create(PathBuf::from("lualatex.log"))?;
        let li = LogItem::Book(LogItemBook {
            title: book.title.clone(),
            status: ELogType::Lualatex(count as u32),
        });
        tx.send(li).await?;
        let child = Command::new("lualatex")
            .arg("--interaction=nonstopmode")
            .arg("main.tex")
            .kill_on_drop(true)
            // .stdout(Stdio::piped())
            .stdout(fout)
            .current_dir(&book.builddir)
            .spawn()?;

        log::info!(
            "{}:{} run lualatex for book {}",
            file!(),
            line!(),
            &book.title
        );
        let output = &child.wait_with_output().await?;

        if !(output.status.success()) {
            success = false;
            break;
        }
        if !needs_rerun(book.builddir.clone())? {
            success = true;
            break;
        }
        count = count + 1;
    }

    log::info!(
        "{}:{} success : {} ; book {}",
        file!(),
        line!(),
        &success,
        &book.title
    );

    if !success {
        let li = LogItem::Book(LogItemBook {
            title: book.title.clone(),
            status: ELogType::Failed,
        });
        log::info!("tx.send final error result for {} ", &book.title);
        tx.send(li).await?;
        return Ok(());
    }

    {
        let mut pto = PathBuf::from(&world.builddir);
        pto.push("delivery");
        pto.push(format!("{}.pdf", &book.pdfname));

        let fout = File::create(PathBuf::from("lualatex.log"))?;
        let li = LogItem::Book(LogItemBook {
            title: book.title.clone(),
            status: ELogType::Ps2pdf,
        });
        tx.send(li).await?;
        let child = Command::new("ps2pdf")
            .arg("main.pdf")
            .arg(pto.to_str().unwrap())
            .kill_on_drop(true)
            // .stdout(Stdio::piped())
            .stdout(fout)
            .current_dir(&book.builddir)
            .spawn()?;

        log::info!("{}:{} run ps2pdf for {} ", file!(), line!(), &book.title);
        log::info!("{}:{} {:?}", file!(), line!(), &child);
        let output = &child.wait_with_output().await?;
        if !(output.status.success()) {
            success = false;
            log::error!("pdf2pdf failed for {}", &book.title);
        }
    }

    let li = LogItem::Book(LogItemBook {
        title: book.title.clone(),
        status: if success {
            ELogType::Success
        } else {
            ELogType::Failed
        },
    });
    log::info!("tx.send final result for book {}", &book.title);
    tx.send(li).await?;

    Ok(())
}
