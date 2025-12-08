use std::path::PathBuf;

use yamake::model as M;
use yamake::rules::lilypond_rules::ly_file::Lyfile;
use yamake::rules::lilypond_rules::lytex_file::Lyoutputfile;

use yamake::rules::tex_rules::pdf_file::Pdffile;
use yamake::rules::tex_rules::tex_file::Texfile;

use crate::model::use_model::World;

pub(crate) async fn make_graph(world: World) -> Result<(), Box<dyn std::error::Error>> {
    let mut g = M::G::new(world.songdir.clone(), world.builddir.clone())?;

    for song in &world.songs {
        {
            let mut p = PathBuf::from(&song.srcdir);
            p.push("body.tex");
            g.add_node(Texfile::new(p)?)?;
        }
        {
            let mut p = PathBuf::from(&song.srcdir);
            p.push("add.tikz");
            g.add_node(Texfile::new(p)?)?;
        }
        {
            let mut p = PathBuf::from(&song.srcdir);
            p.push("song.yml");
            g.add_node(Texfile::new(p)?)?;
        }
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
