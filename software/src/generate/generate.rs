use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use handlebars::Handlebars;
use serde_json::json;

use crate::config::model::World;

pub fn generate(world: &World) -> Result<(), Error> {
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
            // let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

            //             write!(
            //                 output,
            //                 "
            // % import preamble first
            // \\def\\songtitle{{ {} }}
            // \\def\\songauthor{{ {} }}
            // \\renewcommand{{\\makesongtitle}}{{\\xxmakesongtitle{{\\songtitle}}{{\\songauthor}} }}
            // \\renewcommand{{\\songlastupdate}}{{ {} }}
            // \\renewcommand{{\\songtoday}}{{ {} }}
            // ",
            //                 song.title, song.author, song.date, today
            //             )?;

            write!(output, "% length of structure : {}\n", song.structure.len())?;

            // let mut cumul = 0;

            //             for item in song.structure.iter() {
            //                 write!(output, "% structure item name {}", &item.section_id)?;
            //                 match &item.content {
            //                     StructureItemContent::Chords(chords) => {
            //                         let nbcols = std::cmp::min(4, chords.len());
            //                         let firstcol = (0..(chords.len() / nbcols))
            //                             .map(|c| format!("{}-1", c + 1))
            //                             .collect::<Vec<_>>()
            //                             .join(",");
            //                         let colspec = (0..nbcols).map(|_| "C").collect::<Vec<_>>().join("|");
            //                         write!(
            //                             output,
            //                             r###"\
            // \newcommand{{\xxxgrid{section_id}}}{{
            // \begin{{NiceTabular}}{{>{{\raggedright}}p{{0.5cm}}{colspec}}}
            // \CodeBefore
            // \rowcolor{{\lolocolor{section_type}!100}}{{1-{nrows}}}
            // \cellcolor{{white}}{{{firstcol}}}
            // \Body
            // "###,
            //                             section_id = &item.section_id,
            //                             section_type = &item.section_type,
            //                             nrows = (chords.len() / nbcols),
            //                             firstcol = firstcol,
            //                             colspec = colspec
            //                         )?;

            //                         for (index, c) in chords.iter().enumerate() {
            //                             if index % nbcols == 0 {
            //                                 write!(output, "\\tiny{{{index}}} & ", index = cumul + index + 1)?;
            //                             }
            //                             write!(output, "\\chord{} ", c)?;
            //                             // write!(output, "chord{} ", c)?;
            //                             if (index + 1) % nbcols == 0 {
            //                                 write!(output, "\\\\ \n")?;
            //                             } else {
            //                                 write!(output, "& ")?;
            //                             }
            //                         }
            //                         write!(output, "\n\\end{{NiceTabular}} \n")?;
            //                         write!(output, "}}\n")?;

            //                         write!(
            //                             output,
            //                             "\\newcommand{{\\xxxtext{section_id}}}{{ \\{section_type}{{xxx}} {{ {text}  }} }}\n",
            //                             section_id = &item.section_id,section_type=&item.section_type,
            //                             text = &item.text
            //                         )?;

            //                         cumul += chords.len();
            //                     }
            //                 }
            //             }

            let mut reg = Handlebars::new();

            let template =
                String::from_utf8(include_bytes!("../../others/texfiles/struct.tex").to_vec())
                    .unwrap();

            {
                let j = json!(&song);
                log::info!("{}", j.to_string());
            }
            reg.register_template_string("t1", template).unwrap();

            // let mut input_data = BTreeMap::new();
            // input_data.insert("songtitle".to_string(), &song.title);
            // input_data.insert("songauthor".to_string(), &song.author);
            // let chords = &song
            //     .structure
            //     .iter()
            //     .filter_map(|s| match &s.content {
            //         Chords(c) => Some(c),
            //         _ => None,
            //     })
            //     .collect::<Vec<_>>();
            // input_data.insert("chords".to_string(), &song);
            {
                // let output_data = &reg.render_template(template.as_str(), &input_data).unwrap();
                let output_data = reg.render("t1", song).unwrap();
                let _ = output.write(output_data.as_bytes()).unwrap();
            };
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

    Ok(())
}
