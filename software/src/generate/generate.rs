use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::config::model::StructureItemContent;
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
        let bytes = include_bytes!("../../others/shfiles/make_gdrive.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("make_gdrive.sh");
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

            write!(output, "% length of structure : {}", song.structure.len())?;

            let mut cumul = 0;

            for item in song.structure.iter() {
                write!(output, "% structure item name {}", &item.texname)?;
                match &item.content {
                    StructureItemContent::Chords(chords) => {
                        write!(
                            output,
                            r###"\
\newcommand{{\xxxgrid{texname}}}{{                            
\begin{{NiceTabular}}{{p{{0.1cm}}C|C|C|C}}
\CodeBefore
\rowcolor{{\lolocolor{sectiontype}!100}}{{1-{nrows}}}
%\cellcolor{{white}}{{1-1,2-1,3-1,4-1}}
\Body
"###,
                            texname = &item.texname,
                            sectiontype = &item.sectiontype,
                            nrows = (chords.len() / 4)
                        )?;
                        for (index, c) in chords.iter().enumerate() {
                            if index % 4 == 0 {
                                write!(output, "\\tiny{{{index}}} & ", index = cumul + index + 1)?;
                            }
                            write!(output, "\\chord{} ", c)?;
                            // write!(output, "chord{} ", c)?;
                            if (index + 1) % 4 == 0 {
                                write!(output, "\\\\ \n")?;
                            } else {
                                write!(output, "& ")?;
                            }
                        }
                        write!(output, "\n\\end{{NiceTabular}} \n")?;
                        write!(output, "}}\n")?;
                        cumul += chords.len();
                    }
                }
            }
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
