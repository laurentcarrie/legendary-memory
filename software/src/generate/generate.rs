use crate::errors::MyError;
use crate::generate::handlebars_helpers::get_handlebar;
use crate::model::model::World;
use std::fs::File;
use std::io::Write;
// use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::helpers::io::{create_dir_all, write};

pub fn generate(world: &World) -> Result<(), MyError> {
    {
        let bytes = include_bytes!("../../others/shfiles/make_lytex.sh");
        let mut p: PathBuf = world.builddir.clone();
        let () = create_dir_all(&p)?;
        p.push("make_lytex.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
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
        let bytes = include_bytes!("../../others/shfiles/omake.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("omake.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
        // let str_p = p
        //     .to_str()
        //     .ok_or(MyError::MessageError("huh ?".to_string()))?;
        // let mut perms = std::fs::metadata(str_p)?.permissions();
        // perms.set_readonly(true);
        // perms.set_mode(5u32);
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
        let bytes = include_bytes!("../../others/shfiles/make_pdf.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("make_pdf.sh");
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
        let bytes = include_bytes!("../../others/shfiles/make_wav.sh");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("make_wav.sh");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes)?;
    }
    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/preamble.tex");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("songs");
        p.push("preamble.tex");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes_preamble_tex)?;
    }
    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/preamble.tex");
        let mut p: PathBuf = world.builddir.clone();
        p.push("books");
        let _ = create_dir_all(&p)?;
        p.push("preamble.tex");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes_preamble_tex)?;
    }

    {
        let bytes_preamble_tex = include_bytes!("../../others/texfiles/main.tex");
        let mut p: PathBuf = world.builddir.clone();
        p.push("songs");
        let _ = create_dir_all(&p)?;
        p.push("main.tex");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes_preamble_tex)?;
    }

    {
        let bytes_chords_tex = include_bytes!("../../others/texfiles/chords.tex");
        {
            let mut p: PathBuf = world.builddir.clone();
            let _ = create_dir_all(&p)?;
            p.push("songs");
            p.push("chords.tex");
            log::debug!("write {}", p.display());
            let _ = write(&p, bytes_chords_tex)?;
        }
        {
            let mut p: PathBuf = world.builddir.clone();
            let _ = create_dir_all(&p)?;
            p.push("books");
            p.push("chords.tex");
            log::debug!("write {}", p.display());
            let _ = write(&p, bytes_chords_tex)?;
        }
    }
    {
        let bytes_chords_tex = include_bytes!("../../others/lyfiles/macros.ly");
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("songs");
        p.push("macros.ly");
        log::debug!("write {}", p.display());
        let _ = write(&p, bytes_chords_tex)?;
    }

    {
        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("songs");
        p.push("sections.tex");
        let mut output = File::create(p)?;
        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/sections.tex").to_vec())?;

        let mut h = get_handlebar()?;
        h.register_template_string("t1", template)?;
        // let sections: Vec<Section> = world.sections.iter().map(|x| x.1.clone()).collect();
        // let j = handlebars::to_json(&sections);
        let output_data = h.render("t1", world)?;
        let _ = output.write(output_data.as_bytes())?;

        let mut p: PathBuf = world.builddir.clone();
        let _ = create_dir_all(&p)?;
        p.push("books");
        p.push("sections.tex");
        let mut output = File::create(p)?;
        let _ = output.write(output_data.as_bytes())?;
    }

    {
        for song in &world.songs {
            // {
            //     let mut output = File::create("debug.json")?;
            //     write!(output, "{}", serde_json::to_string(&song)?)?;
            // }
            let mut p: PathBuf = song.builddir.clone();
            let _ = create_dir_all(&p)?;
            p.push("data.tex");
            log::debug!("write {}", p.display());
            let mut output = File::create(&p).or_else(|e| {
                Err(MyError::MessageError(format!(
                    "{:?} could not create {:?}",
                    e,
                    &p.to_str()
                )))
            })?;
            write!(output, "% length of structure : {}\n", song.structure.len())?;

            let template =
                String::from_utf8(include_bytes!("../../others/texfiles/data.tex").to_vec())?;

            let mut h = get_handlebar()?;
            h.register_template_string("t1", template)?;
            let output_data = h.render("t1", song)?;
            let _ = output.write(output_data.as_bytes())?;
        }
    }

    {
        let mut p: PathBuf = world.builddir.clone();
        p.push("delivery");
        let _ = create_dir_all(&p)?;
    }

    // {
    //     let mut p: PathBuf = world.builddir.clone();
    //     let _ = create_dir_all(&p)?;
    //     p.push("macros.ly");
    //     log::debug!("write {}", p.display());
    //     let mut output = File::create(p)?;
    //     let data = make_macros();
    //     write!(output, "{}", data)?;
    // }

    Ok(())
}
