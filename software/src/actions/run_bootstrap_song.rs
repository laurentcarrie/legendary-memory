use crate::actions::mount::{mount_from_data, mount_from_file};
use crate::generate::handlebars_helpers::get_handlebar;
use crate::helpers::io::{
    create_dir_all,
    // read_to_string, read_to_vec_u8, write,
    write_string,
};
use crate::helpers::path::make_path;
use crate::model::use_model as M;
use std::path::PathBuf;

pub async fn run(
    world: M::World,
    song: M::Song,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut target = song.builddir.clone();
    target.push("bootstrap");

    let mut changed = 0;

    create_dir_all(&song.builddir)?;
    changed += mount_from_data(
        include_bytes!("../../others/texfiles/preamble.tex").to_vec(),
        make_path(song.builddir.clone(), vec!["preamble.tex"]),
    )?;
    changed += mount_from_data(
        include_bytes!("../../others/texfiles/main.tex").to_vec(),
        make_path(song.builddir.clone(), vec!["main.tex"]),
    )?;
    assert!(make_path(song.builddir.clone(), vec!["main.tex"]).exists());
    changed += mount_from_data(
        include_bytes!("../../others/texfiles/tikzlibraryspline.code.tex").to_vec(),
        make_path(song.builddir.clone(), vec!["tikzlibraryspline.code.tex"]),
    )?;

    changed += mount_from_data(
        include_bytes!("../../others/texfiles/chords.tex").to_vec(),
        make_path(song.builddir.clone(), vec!["chords.tex"]),
    )?;

    {
        let template =
            String::from_utf8(include_bytes!("../../others/lyfiles/macros.ly").to_vec())?;
        let mut h = get_handlebar()?;
        h.register_template_string("t1", &template)?;
        // let sections: Vec<Section> = world.sections.iter().map(|x| x.1.clone()).collect();
        // let j = handlebars::to_json(&sections);
        let output_data = h.render("t1", &song)?;
        changed += mount_from_data(
            output_data.into(),
            make_path(song.builddir.clone(), vec!["macros.ly"]),
        )?;
    }

    {
        let mut p: PathBuf = song.builddir.clone();
        p.push("data.tex");

        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/data.tex").to_vec())?;

        let mut h = get_handlebar()?;
        h.register_template_string("t1", template)?;
        let output_data = h.render("t1", &song)?;
        changed += mount_from_data(
            output_data.into(),
            make_path(song.builddir.clone(), vec!["data.tex"]),
        )?;
    }

    {
        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/sections.tex").to_vec())?;
        let mut h = get_handlebar()?;
        h.register_template_string("t1", template)?;
        let output_data = h.render("t1", &world)?;
        changed += mount_from_data(
            output_data.into(),
            make_path(song.builddir.clone(), vec!["sections.tex"]),
        )?;
    }

    let srcdir = PathBuf::from(song.srcdir.clone());

    // mount files
    {
        let mut pfrom = PathBuf::from(song.srcdir.clone());
        pfrom.push("lyrics");
        create_dir_all(&pfrom)?;
        let mut pto = song.builddir.clone();
        pto.push("lyrics");
        create_dir_all(&pto)?;
    }
    changed += mount_from_file(
        make_path(srcdir.clone(), vec!["body.tex"]),
        make_path(song.builddir.clone(), vec!["body.tex"]),
    )?;

    changed += mount_from_file(
        make_path(srcdir.clone(), vec!["add.tikz"]),
        make_path(song.builddir.clone(), vec!["add.tikz"]),
    )?;

    for lyfile in &song.lilypondfiles {
        changed += mount_from_file(
            make_path(srcdir.clone(), vec![lyfile]),
            make_path(song.builddir.clone(), vec![lyfile]),
        )?;
    }

    let lyrics_ids = &song
        .structure
        .iter()
        .filter_map(|item| match &item.item {
            M::StructureItemContent::ItemChords(s) => Some(s.section_id.clone()),
            M::StructureItemContent::ItemRef(s) => Some(s.section_id.clone()),
            M::StructureItemContent::ItemHRule(_) => None,
            M::StructureItemContent::ItemNewColumn => None,
        })
        .collect::<Vec<_>>();

    for id in lyrics_ids {
        let pfrom = make_path(srcdir.clone(), vec!["lyrics", format!("{id}.tex").as_str()]);
        if !pfrom.exists() {
            write_string(&pfrom, &"\\color{red}{put something here}".to_string())?;
        }
        let pto = make_path(
            song.builddir.clone(),
            vec!["lyrics", format!("{id}.tex").as_str()],
        );

        changed += mount_from_file(pfrom, pto)?
    }

    {
        // song.tikz
        // for (id, song) in world.songs.iter().enumerate() {
        // let id = song.id;
        //@ todo id of song : enumerate
        // let id = 33;

        let songtikzuserinput = {
            let mut p: PathBuf = srcdir.clone();
            p.push("add.tikz");
            std::fs::read_to_string(p)?
        };

        let template =
            String::from_utf8(include_bytes!("../../others/texfiles/song.tikz").to_vec())?;

        #[derive(serde::Serialize)]
        struct Xxx<'a> {
            song: &'a M::Song,
            songtikzuserinput: String,
            // id: u32,
        }
        let data = Xxx {
            song: &song,
            songtikzuserinput,
            // id: id.try_into()?,
        };
        log::debug!("generate song.tikz");
        let mut h = get_handlebar()?;
        h.register_template_string("t1", template)?;
        let output_data = h.render("t1", &data)?;
        changed += mount_from_data(
            output_data.into(),
            make_path(song.builddir.clone(), vec!["song.tikz"]),
        )?;
    }

    log::info!("bootstrap changed : {}", changed);
    write_string(&target, &"".to_string())?;

    if changed > 0 {
        Ok(M::BuildType::Rebuilt(target))
    } else {
        Ok(M::BuildType::NotTouched(target))
    }
}
