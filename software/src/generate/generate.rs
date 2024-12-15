use crate::config::model::World;
use crate::generate::handlebars_helpers::get_handlebar;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

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
        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("songs");
        p.push("sections.tex");
        let mut output = File::create(p)?;
        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/sections.tex").to_vec())
                .unwrap();

        let mut h = get_handlebar()?;
        h.register_template_string("t1", template).unwrap();
        // let sections: Vec<Section> = world.sections.iter().map(|x| x.1.clone()).collect();
        // let j = handlebars::to_json(&sections);
        let output_data = h.render("t1", world).unwrap();
        let _ = output.write(output_data.as_bytes()).unwrap();

        let mut p: PathBuf = world.builddir.clone();
        let _ = fs::create_dir_all(&p)?;
        p.push("books");
        p.push("sections.tex");
        let mut output = File::create(p)?;
        let _ = output.write(output_data.as_bytes()).unwrap();
    }

    {
        for song in &world.songs {
            // {
            //     let mut output = File::create("debug.json")?;
            //     write!(output, "{}", serde_json::to_string(&song)?)?;
            // }
            let mut p: PathBuf = song.builddir.clone();
            let _ = fs::create_dir_all(&p)?;
            p.push("data.tex");
            log::debug!("write {}", p.display());
            let mut output = File::create(p)?;
            write!(output, "% length of structure : {}\n", song.structure.len())?;

            let template =
                String::from_utf8(include_bytes!("../../others/texfiles/data.tex").to_vec())
                    .unwrap();

            let mut h = get_handlebar()?;
            h.register_template_string("t1", template).unwrap();
            let output_data = h.render("t1", song).unwrap();
            let _ = output.write(output_data.as_bytes()).unwrap();
        }
    }

    {
        let mut p: PathBuf = world.builddir.clone();
        p.push("delivery");
        let _ = fs::create_dir_all(&p)?;
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
