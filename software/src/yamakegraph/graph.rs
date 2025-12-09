use std::path::PathBuf;

use yamake::model as M;
use yamake::rules::lilypond_rules::ly_file::Lyfile;
use yamake::rules::lilypond_rules::lytex_file::Lyoutputfile;

use yamake::rules::tex_rules::pdf_file::Pdffile;
use yamake::rules::tex_rules::tex_file::Texfile;

use crate::model::config::build_relative_path_of_source_absolute_path;

use crate::model::use_model::{StructureItemContent, World};

pub(crate) async fn make_graph(world: World) -> Result<(), Box<dyn std::error::Error>> {
    let mut g = M::G::new(world.songdir.clone(), world.builddir.clone())?;

    enum ThingToMount {
        Tex,
        Ly,
    }

    for song in &world.songs {
        // log::info!("{:?}", song);
        let mut items_to_mount: Vec<(String, ThingToMount)> = vec![];

        items_to_mount.push(("body.tex".to_string(), ThingToMount::Tex));

        {
            for s in &song.texfiles {
                items_to_mount.push((s.to_string(), ThingToMount::Tex))
            }
            {
                for s in &song.structure {
                    match &s.item {
                        StructureItemContent::ItemChords(c) => items_to_mount.push((
                            format!("lyrics/{}.tex", c.section_id).to_string(),
                            ThingToMount::Tex,
                        )),
                        StructureItemContent::ItemRef(r) => items_to_mount.push((
                            format!("lyrics/{}.tex", r.section_id).to_string(),
                            ThingToMount::Tex,
                        )),
                        StructureItemContent::ItemHRule(_)
                        | StructureItemContent::ItemNewColumn => (),
                    }
                }
            }

            for (a, b) in items_to_mount {
                let mut p = song.srcdir.clone();
                p.push(a);
                let p2 = build_relative_path_of_source_absolute_path(
                    &world.songdir.clone().as_path(),
                    world.builddir.clone().as_path(),
                    p.clone(),
                )?;
                match b {
                    ThingToMount::Tex => g.add_node(Texfile::new(p)?)?,
                    ThingToMount::Ly => g.add_node(Lyfile::new(p)?)?,
                };
            }
        }

        // {
        //     let mut p = song.srcdir.clone();
        //     p.push("add.tikz");
        //     let p = build_relative_path_of_source_absolute_path(
        //         &world.songdir.clone().as_path(),
        //         &world.builddir.clone().as_path(),
        //         p,
        //     )?;

        //     g.add_node(Texfile::new(p)?)?;
        // }
        // {
        //     let mut p = song.srcdir.clone();
        //     p.push("song.yml");
        //     let p = build_relative_path_of_source_absolute_path(
        //         &world.songdir.clone().as_path(),
        //         &world.builddir.clone(),
        //         p,
        //     )?;

        //     g.add_node(Texfile::new(p)?)?;
        // }
    }

    match g.make(false, 8).await {
        Ok(ret) => {
            println!("success : {}", ret.success);
            // you can walk the graph and print status of each node
            // for (k, v) in ret.nt {
            //     println!("node {:?} : {:?}", k, v);
            // }
        }
        Err(e) => println!("{}", e.to_string()),
    };

    Ok(())
}
