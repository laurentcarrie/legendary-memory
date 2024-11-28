use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::World;
use crate::emitter::emitter::write_mp;

pub fn generate(world: &World) -> Result<(), Error> {
    // include_bytes!("../../others/shfiles/make_lytex.sh"),
    // include_bytes!("../../others/shfiles/colors.sh"),
    // include_bytes!("../../others/shfiles/make_mpost.sh"),
    // include_bytes!("../../others/shfiles/make_pdf.sh"),
    // include_bytes!("../../others/shfiles/make_wav.sh"),
    // include_bytes!("../../others/shfiles/make_clean.sh"),
    {
        let bytes = include_bytes!("../../others/shfiles/make_lytex.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_lytex.sh");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes).unwrap();
    }
    {
        let bytes = include_bytes!("../../others/shfiles/colors.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("colors.sh");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes).unwrap();
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_clean.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_clean.sh");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes).unwrap();
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_mpost.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_mpost.sh");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes).unwrap();
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_pdf.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_pdf.sh");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes).unwrap();
    }
    {
        let bytes = include_bytes!("../../others/shfiles/make_wav.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_wav.sh");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes).unwrap();
    }
    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/preamble.tex");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("songs");
        p.push("preamble.tex");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes_preamble_tex).unwrap();
    }
    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/preamble.tex");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("books");
        p.push("preamble.tex");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes_preamble_tex).unwrap();
    }

    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/main.tex");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("songs");
        p.push("main.tex");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes_preamble_tex).unwrap();
    }

    {
        let bytes_chords_tex = include_bytes!("../../others/texfiles/chords.tex");
        {
            let mut p: PathBuf = world.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("songs");
            p.push("chords.tex");
            log::debug!("write {}", p.display());
            let _ = fs::write(&p, bytes_chords_tex).unwrap();
        }
        {
            let mut p: PathBuf = world.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("books");
            p.push("chords.tex");
            log::debug!("write {}", p.display());
            let _ = fs::write(&p, bytes_chords_tex).unwrap();
        }
    }
    {
        let bytes_chords_tex = include_bytes!("../../others/lyfiles/macros.ly");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("songs");
        p.push("macros.ly");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes_chords_tex).unwrap();
    }

    {
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("data.tex");
            log::debug!("write {}", p.display());
            let mut output = File::create(p)?;
            //let data = make_preamble();
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

            write!(
                output,
                "
% import preamble first
\\def\\songtitle{{ {} }}
\\def\\songauthor{{ {} }}
\\renewcommand{{\\makesongtitle}}{{\\xxmakesongtitle{{\\songtitle}}{{\\songauthor}} }}
\\renewcommand{{\\songlastupdate}}{{ {} }}
\\renewcommand{{\\songtoday}}{{ {} }}
",
                song.title, song.author, song.date, today
            )?;
        }
    }
    // {
    //     let mut p: PathBuf = world.builddir.clone();
    //     let _ = fs::create_dir_all(&p)?;
    //     p.push("macros.ly");
    //     log::debug!("write {}", p.display());
    //     let mut output = File::create(p)?;
    //     let data = make_macros();
    //     write!(output, "{}", data)?;
    // }
    {
        for song in world.songs.iter() {
            for section in song.sections.iter() {
                write_mp(&section, &song)?;
            }
        }
    }
    Ok(())
}
