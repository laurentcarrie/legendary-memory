use crate::model::use_model as M;
use petgraph::Graph;

use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum EArtefact {
    BootstrapSong(M::Song),
    BootstrapBook(M::Book),
    SongPdf(M::Song),
    DeliverySongPdf(M::Song),
    MountedFile(M::Song, String),
    LySnippet(M::Song, String),
    Midi(M::Song, String),
    Wav(M::Song, String),
    BookPdf(M::Book),
    DeliveryBookPdf(M::Book),
    Broken(PathBuf),
    All,
}

impl std::fmt::Debug for EArtefact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EArtefact::SongPdf(song) => write!(f, "SongPdf {} @ {}", song.author, song.title),
            EArtefact::DeliverySongPdf(song) => {
                write!(f, "DeliverySongPdf {} @ {} ", song.author, song.title)
            }
            EArtefact::MountedFile(song, tex) => {
                write!(f, "MountedFile {} @ {} ; {}", song.author, song.title, tex)
            }
            EArtefact::LySnippet(song, ly) => {
                write!(f, "LySnippet {} @ {} : {}", song.author, song.title, ly)
            }
            EArtefact::Midi(song, midi) => {
                write!(f, "Midi({} @ {} ; {})", song.author, song.title, midi)
            }
            EArtefact::Wav(song, wav) => {
                write!(f, "Wav({} @ {} ; {})", song.author, song.title, wav)
            }
            EArtefact::BookPdf(book) => write!(f, "BookPdf({})", book.title),
            EArtefact::DeliveryBookPdf(book) => write!(f, "DeliveryBookPdf({})", book.title),
            EArtefact::All => write!(f, "All"),
            EArtefact::BootstrapSong(song) => {
                write!(f, "Bootstrap {} @ {}", song.author, song.title)
            }
            EArtefact::BootstrapBook(book) => {
                write!(
                    f,
                    "Boostrap book {} (lyrics only: {})",
                    book.title, book.lyrics_only
                )
            }
            EArtefact::Broken(path) => write!(f, "Broken({})", path.display()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Artefact {
    pub kind: EArtefact,
}

impl Artefact {
    pub fn new_pdf_song(song: M::Song) -> Self {
        Artefact {
            kind: EArtefact::SongPdf(song),
        }
    }
    pub fn new_delivery_song(song: M::Song) -> Self {
        Artefact {
            kind: EArtefact::DeliverySongPdf(song),
        }
    }
    pub fn new_pdf_book(book: M::Book) -> Self {
        Artefact {
            kind: EArtefact::BookPdf(book),
        }
    }
    pub fn new_delivery_book(book: M::Book) -> Self {
        Artefact {
            kind: EArtefact::DeliveryBookPdf(book),
        }
    }
    pub fn new_mounted_file(song: M::Song, f: String) -> Self {
        Artefact {
            kind: EArtefact::MountedFile(song, f),
        }
    }
    pub fn new_ly_snippet(song: M::Song, f: String) -> Self {
        Artefact {
            kind: EArtefact::LySnippet(song, f),
        }
    }
    pub fn new_bootstrap_song(song: M::Song) -> Self {
        Artefact {
            kind: EArtefact::BootstrapSong(song),
        }
    }
    pub fn new_bootstrap_book(book: M::Book) -> Self {
        Artefact {
            kind: EArtefact::BootstrapBook(book),
        }
    }
    pub fn new_midi(song: M::Song, file: String) -> Self {
        Artefact {
            kind: EArtefact::Midi(song, file),
        }
    }
    pub fn new_wav(song: M::Song, file: String) -> Self {
        Artefact {
            kind: EArtefact::Wav(song, file),
        }
    }
    pub fn new_broken_song(path: PathBuf) -> Self {
        Artefact {
            kind: EArtefact::Broken(path),
        }
    }
}

// #[cfg(feature = "hgraph")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Route(String);

/// Creates a sample graph representing Viking trade and travel routes.
///
/// # Arguments
/// * `directed` - Whether the graph should be directed (`true`) or undirected (`false`).
///
/// # Returns
/// A `HeterogeneousGraph` with cities as nodes and routes as edges, including attributes.
// #[cfg(feature = "hgraph")]
pub fn build_graph(world: &M::World) -> Result<Graph<Artefact, Route>, Box<dyn std::error::Error>> {
    let mut graph: Graph<Artefact, Route> = Graph::new();

    let all = graph.try_add_node(Artefact {
        kind: EArtefact::All,
    })?;

    let mut songmap: HashMap<(String, String), petgraph::graph::NodeIndex> = HashMap::new();

    for song in &world.songs {
        let bootstrap = graph.try_add_node(Artefact::new_bootstrap_song(song.clone()).into())?;
        let mainpdf = graph.try_add_node(Artefact::new_pdf_song(song.clone()).into())?;
        let _ = graph.try_add_edge(bootstrap, mainpdf, Route("bootstrap".to_string()));
        let deliverypdf = graph.try_add_node(Artefact::new_delivery_song(song.clone()).into())?;
        for t in &song.texfiles {
            let mountedtex =
                graph.try_add_node(Artefact::new_mounted_file(song.clone(), t.clone()).into())?;
            let _ = graph.try_add_edge(mountedtex, mainpdf, Route("lualatex".to_string()));
            let _ = graph.try_add_edge(bootstrap, mountedtex, Route("bootstrap".to_string()));
        }
        for s in &song.structure {
            match &s.item {
                M::StructureItemContent::ItemChords(c) => {
                    let l = graph.try_add_node(Artefact::new_mounted_file(
                        song.clone(),
                        format!("lyrics/{}.tex", c.section_id),
                    ))?;
                    let _ = graph.try_add_edge(l, mainpdf, Route("lualatex".to_string()));
                }
                M::StructureItemContent::ItemRef(c) => {
                    let l = graph.try_add_node(Artefact::new_mounted_file(
                        song.clone(),
                        format!("lyrics/{}.tex", c.section_id),
                    ))?;
                    let _ = graph.try_add_edge(l, mainpdf, Route("lualatex".to_string()));
                }
                M::StructureItemContent::ItemHRule(_) | M::StructureItemContent::ItemNewColumn => {}
            }
        }
        for t in &song.lilypondfiles {
            let lytex =
                graph.try_add_node(Artefact::new_mounted_file(song.clone(), t.clone()).into())?;
            let lysnippet =
                graph.try_add_node(Artefact::new_ly_snippet(song.clone(), t.clone()).into())?;
            let _ = graph.try_add_edge(lytex, lysnippet, Route("lilypond-book".to_string()));
            let _ = graph.try_add_edge(lysnippet, mainpdf, Route("lualatex".to_string()));
            let _ = graph.try_add_edge(bootstrap, lytex, Route("bootstrap".to_string()));
        }
        for m in &song.wavfiles {
            let midi =
                graph.try_add_node(Artefact::new_midi(song.clone(), m.to_string()).into())?;
            let _ = graph.try_add_edge(bootstrap, midi, Route("midi".to_string()));
            let wav = graph.try_add_node(Artefact::new_wav(song.clone(), m.to_string()).into())?;
            let _ = graph.try_add_edge(midi, wav, Route("wav".to_string()));
            let _ = graph.try_add_edge(wav, all, Route("all".to_string()));
        }
        let _ = graph.try_add_edge(bootstrap, mainpdf, Route("delivery".to_string()));
        let _ = graph.try_add_edge(mainpdf, deliverypdf, Route("delivery".to_string()));
        let _ = graph.try_add_edge(deliverypdf, all, Route("all".to_string()));
        songmap.insert((song.author.clone(), song.title.clone()), deliverypdf);
    }

    for song in &world.broken_songs {
        let songjson: petgraph::prelude::NodeIndex =
            graph.try_add_node(Artefact::new_broken_song(song.0.clone()))?;
        let _ = graph.try_add_edge(songjson, all, Route("all".to_string()));
    }

    for book in &world.books {
        let bootstrap: petgraph::prelude::NodeIndex =
            graph.try_add_node(Artefact::new_bootstrap_book(book.clone()).into())?;
        let mainpdf = graph.try_add_node(Artefact::new_pdf_book(book.clone()).into())?;
        let _ = graph.try_add_edge(bootstrap, mainpdf, Route("lualatex".to_string()));
        let deliverypdf = graph.try_add_node(Artefact::new_delivery_book(book.clone()).into())?;
        let _ = graph.try_add_edge(mainpdf, deliverypdf, Route("delivery".to_string()));
        let _ = graph.try_add_edge(deliverypdf, all, Route("all".to_string()));

        for song in &book.songs {
            let nodeid = songmap
                .get(&(song.author.clone(), song.title.clone()))
                .ok_or("could not find song in graph")?;
            let _ = graph.try_add_edge(*nodeid, mainpdf, Route("book depends on song".to_string()));
        }
    }

    for n in graph.node_indices() {
        let node = graph.node_weight(n).ok_or("huh, no node?")?;
        log::info!("{:?} ", node);

        for p in graph.neighbors_directed(n, petgraph::Direction::Incoming) {
            let pnode = graph.node_weight(p).ok_or("huh, no node?")?;
            log::info!("  <--- {:?}", pnode);
        }
    }

    Ok(graph)
}
