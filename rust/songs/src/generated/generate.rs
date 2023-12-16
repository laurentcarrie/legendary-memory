use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::World;
use crate::emitter::emitter::write_mp;
use crate::generated::ly_code::make_macros;
use crate::generated::sh_code::{
    make_colors, make_make_clean, make_make_lytex, make_make_mpost, make_make_pdf, make_make_wav,
};
use crate::generated::tex_code::{make_chords, make_preamble};

pub fn generate(world: &World) -> Result<(), Error> {
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_lytex.sh");
        println!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_lytex();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_clean.sh");
        println!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_clean();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_mpost.sh");
        println!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_mpost();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_pdf.sh");
        println!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_pdf();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_wav.sh");
        println!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_make_wav();
        write!(output, "{}", data)?;
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("colors.sh");
        println!("write {}", p.display());
        let mut output = File::create(p)?;
        let data = make_colors();
        write!(output, "{}", data)?;
    }
    {
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("preamble.tex");
            println!("write {}", p.display());
            let mut output = File::create(p)?;
            let data = make_preamble();
            write!(output, "{}", data)?;
        }
    }
    {
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("chords.tex");
            println!("write {}", p.display());
            let mut output = File::create(p)?;
            let data = make_chords();
            write!(output, "{}", data)?;
        }
    }
    {
        for song in &world.songs {
            let mut p: PathBuf = song.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("data.tex");
            println!("write {}", p.display());
            let mut output = File::create(p)?;
            //let data = make_preamble();
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

            write!(
                output,
                "
% import preamble first
\\def\\songtitle{{ {} }}
\\def\\songauthor{{ {} }}
\\newcommand{{\\makesongtitle}}{{\\xxmakesongtitle{{\\songtitle}}{{\\songauthor}} }}
\\newcommand{{\\songlastupdate}}{{ {} }}
\\newcommand{{\\songtoday}}{{ {} }}
",
                song.title, song.author, song.date, today
            )?;
        }
    }
    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("macros.ly");
        println!("write {}", p.display());
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
