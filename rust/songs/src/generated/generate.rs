use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::World;
use crate::emitter::emitter::write_mp;
use crate::generated::ly_code::make_macros;
use crate::generated::sh_code::{
    make_colors, make_make_clean, make_make_gdrive, make_make_lytex, make_make_mpost,
    make_make_pdf, make_make_wav,
};

pub fn generate(world: &World) -> Result<(), Error> {
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_lytex.sh");
        log::info!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_lytex();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_clean.sh");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_clean();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_mpost.sh");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_mpost();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_pdf.sh");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_pdf();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_wav.sh");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_wav();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("colors.sh");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_colors();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_gdrive.sh");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_gdrive();
        write!(output, "{}", data)?;
    }
    // {
    //     for song in &world.songs {
    //         let mut p: PathBuf = song.builddir.clone();
    //         let _ = fs::create_dir_all(&p)?;
    //         p.push("preamble.tex");
    //         log::debug!("write {}", p.display());
    //         let mut output = File::create(p)?;
    //         let data = make_preamble();
    //         write!(output, "{}", data)?;
    //     }
    // }
    // {
    //     for song in &world.songs {
    //         let mut p: PathBuf = song.builddir.clone();
    //         let _ = fs::create_dir_all(&p)?;
    //         p.push("chords.tex");
    //         log::debug!("write {}", p.display());
    //         let mut output = File::create(p)?;
    //         let data = make_chords();
    //         write!(output, "{}", data)?;
    //     }
    // }

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
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/preamble.tex");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("songs");
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
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        // p.push("books");
        p.push("chords.tex");
        log::debug!("write {}", p.display());
        let _ = fs::write(&p, bytes_chords_tex).unwrap();
    }

    {
        let bytes_chords_tex = include_bytes!("../../others/texfiles/chords.tex");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("songs");
        p.push("chords.tex");
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
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("macros.ly");
        log::debug!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_macros();
        write!(output, "{}", data)?;
    }
    {
        for song in world.songs.iter() {
            for section in song.sections.iter() {
                write_mp(&section, &song)?;
            }
        }
    }
    Ok(())
}
