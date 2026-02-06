use handlebars::Handlebars;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use yamake::model::{Edge, ExpandError, ExpandResult, GNode, GRootNode};

use super::{LilypondFile, LyTexFile, PdfFile, SongTikz, TexFile, TexOfLilypond};
use crate::helpers::register_helpers;
use crate::model::{SectionItem, Song};
use crate::settings::Settings;

const SONG_TIKZ_TEMPLATE: &str = include_str!("../resources/texfiles/song.tikz");
const PREAMBLE_TEMPLATE: &str = include_str!("../resources/texfiles/preamble.tex");
const TIKZ_SPLINE_LIB: &str = include_str!("../resources/texfiles/tikzlibraryspline.code.tex");
const SECTIONS_TEMPLATE: &str = include_str!("../resources/texfiles/sections.tex");
const CHORDS_TEX: &str = include_str!("../resources/texfiles/chords.tex");
const DATA_TEMPLATE: &str = include_str!("../resources/texfiles/data.tex");
const MACROS_LY_TEMPLATE: &str = include_str!("../resources/lyfiles/macros.ly");

pub struct SongYml {
    pub path: PathBuf,
}

impl SongYml {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl GRootNode for SongYml {
    fn tag(&self) -> String {
        "song.yml".to_string()
    }

    fn pathbuf(&self) -> PathBuf {
        self.path.clone()
    }

    fn expand(&self, sandbox: &Path, _predecessors: &[&(dyn GNode + Send + Sync)]) -> ExpandResult {
        // Get the directory containing song.yml
        let parent_dir = self.path.parent().unwrap_or(Path::new(""));

        // Read the song.yml file to get song data (needed for main.tex and templates)
        let song_yml_path = sandbox.join(&self.path);
        let mut song: Song = match std::fs::read_to_string(&song_yml_path) {
            Ok(content) => match serde_yaml::from_str(&content) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("Failed to parse song.yml: {e}");
                    return Err(ExpandError::Other(e.to_string()));
                }
            },
            Err(e) => {
                log::error!("Failed to read song.yml: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        // Load settings from settings.yml at sandbox root
        let settings = Settings::load(sandbox).map_err(ExpandError::Other)?;
        let section_colors = &settings.colors;

        // Resolve colors for Chords items using color_of_section
        for item in &mut song.structure {
            if let SectionItem::Chords(chords) = &mut item.item {
                let color = section_colors
                    .color_of_section(&chords.section_type, chords.color.as_deref())
                    .map_err(ExpandError::Other)?;
                chords.color = Some(color);
            }
        }

        // Build a map of Chords id -> color for Ref color resolution
        let chords_colors: std::collections::HashMap<String, String> = song
            .structure
            .iter()
            .filter_map(|item| match &item.item {
                SectionItem::Chords(chords) => {
                    Some((item.id.clone(), chords.color.clone().unwrap_or_default()))
                }
                _ => None,
            })
            .collect();

        // Validate Ref items and fill in missing colors from linked Chords
        for item in &mut song.structure {
            if let SectionItem::Ref(ref_section) = &mut item.item {
                if let Some(linked_color) = chords_colors.get(&ref_section.link) {
                    // Fill in color from linked Chords if not specified
                    if ref_section.color.is_none() {
                        ref_section.color = Some(linked_color.clone());
                    }
                } else {
                    let error_msg = format!(
                        "Ref '{}' links to '{}' but no Chords item with that id exists",
                        item.id, ref_section.link
                    );
                    log::error!("{error_msg}");
                    return Err(ExpandError::Other(error_msg));
                }
            }
        }

        // Convert Song to JSON for handlebars templates
        let song_data = match serde_json::to_value(&song) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to serialize song data: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        // Extract unique section types for sections.tex
        let mut section_types: HashSet<String> = HashSet::new();
        for item in &song.structure {
            match &item.item {
                SectionItem::Chords(chords) => {
                    section_types.insert(chords.section_type.clone());
                }
                SectionItem::Ref(ref_section) => {
                    if let Some(ref st) = ref_section.section_type {
                        section_types.insert(st.clone());
                    }
                }
                _ => {}
            }
        }

        // Build sections data with default colors
        let sections: Vec<serde_json::Value> = section_types
            .into_iter()
            .map(|id| {
                let color = match id.as_str() {
                    "intro" => "yellow!20",
                    "couplet" => "green!20",
                    "coupletb" => "green!10",
                    "refrain" => "blue!20",
                    "pont" => "orange!20",
                    "outro" => "red!20",
                    _ => "gray!20",
                };
                serde_json::json!({"id": id, "color": color})
            })
            .collect();

        // Create main.tex path relative to song.yml location
        let tex_path = parent_dir.join("main.tex");
        let pdf_path = parent_dir.join("main.pdf");

        // Create the main.tex file in the sandbox
        let tex_full_path = sandbox.join(&tex_path);
        if let Some(parent) = tex_full_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let tex_content = r#"\documentclass{article}
\PassOptionsToPackage{x11names}{xcolor}
\usepackage{tikz}
\input{preamble}
\input{chords}
\input{sections}
\input{data}
\begin{document}
\input{body}
\end{document}
"#
        .to_string();
        if let Err(e) = std::fs::write(&tex_full_path, &tex_content) {
            log::error!("Failed to write main.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create song.tikz file using handlebars template
        let tikz_path = parent_dir.join("song.tikz");
        let tikz_full_path = sandbox.join(&tikz_path);

        // Render the templates
        let mut handlebars = Handlebars::new();
        register_helpers(&mut handlebars);
        let template_data =
            serde_json::json!({"song": song_data, "sections": sections, "settings": settings});

        // Render song.tikz
        let tikz_content = match handlebars.render_template(SONG_TIKZ_TEMPLATE, &template_data) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to render song.tikz template: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        if let Err(e) = std::fs::write(&tikz_full_path, &tikz_content) {
            log::error!("Failed to write song.tikz: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create preamble.tex file using handlebars template
        let preamble_path = parent_dir.join("preamble.tex");
        let preamble_full_path = sandbox.join(&preamble_path);
        let preamble_content = match handlebars.render_template(PREAMBLE_TEMPLATE, &template_data) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to render preamble.tex template: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        if let Err(e) = std::fs::write(&preamble_full_path, &preamble_content) {
            log::error!("Failed to write preamble.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create tikzlibraryspline.code.tex file
        let spline_path = parent_dir.join("tikzlibraryspline.code.tex");
        let spline_full_path = sandbox.join(&spline_path);
        if let Err(e) = std::fs::write(&spline_full_path, TIKZ_SPLINE_LIB) {
            log::error!("Failed to write tikzlibraryspline.code.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create sections.tex file using handlebars template
        let sections_path = parent_dir.join("sections.tex");
        let sections_full_path = sandbox.join(&sections_path);
        let sections_content = match handlebars.render_template(SECTIONS_TEMPLATE, &template_data) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to render sections.tex template: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        if let Err(e) = std::fs::write(&sections_full_path, &sections_content) {
            log::error!("Failed to write sections.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create chords.tex file
        let chords_path = parent_dir.join("chords.tex");
        let chords_full_path = sandbox.join(&chords_path);
        if let Err(e) = std::fs::write(&chords_full_path, CHORDS_TEX) {
            log::error!("Failed to write chords.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create data.tex file using handlebars template
        let data_path = parent_dir.join("data.tex");
        let data_full_path = sandbox.join(&data_path);
        let data_content = match handlebars.render_template(DATA_TEMPLATE, &template_data) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to render data.tex template: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        if let Err(e) = std::fs::write(&data_full_path, &data_content) {
            log::error!("Failed to write data.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create macros.ly file using handlebars template
        let macros_ly_path = parent_dir.join("macros.ly");
        let macros_ly_full_path = sandbox.join(&macros_ly_path);
        let macros_ly_content = match handlebars.render_template(MACROS_LY_TEMPLATE, &template_data)
        {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to render macros.ly template: {e}");
                return Err(ExpandError::Other(e.to_string()));
            }
        };

        if let Err(e) = std::fs::write(&macros_ly_full_path, &macros_ly_content) {
            log::error!("Failed to write macros.ly: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create lyrics/main.tex file with inputs for each Chords/Ref section
        let lyrics_tex_path = parent_dir.join("lyrics").join("main.tex");
        let lyrics_tex_full_path = sandbox.join(&lyrics_tex_path);
        if let Some(parent) = lyrics_tex_full_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // Build the lyrics/main.tex content
        let mut lyrics_inputs = String::new();
        for item in &song.structure {
            match &item.item {
                SectionItem::Chords(chords) => {
                    let color = chords.color.as_deref().unwrap_or("white");
                    lyrics_inputs.push_str(&format!(
                        "\\colorbox{{{}}}{{\\textbf{{{}}}}}\n\n\\input{{{}}}\n\\vspace{{1em}}\n\n",
                        color, chords.title, item.id
                    ));
                }
                SectionItem::Ref(ref_section) => {
                    let color = ref_section.color.as_deref().unwrap_or("white");
                    lyrics_inputs.push_str(&format!(
                        "\\colorbox{{{}}}{{\\textbf{{{}}}}}\n\n\\input{{{}}}\n\\vspace{{1em}}\n\n",
                        color, ref_section.title, item.id
                    ));
                }
                _ => {}
            }
        }

        let lyrics_tex_content = format!(
            r#"\documentclass{{article}}
\usepackage[left=1cm,right=1cm,top=1cm,bottom=2cm]{{geometry}}
\usepackage{{fontspec}}
\setmainfont{{Garamond Libre}}
\usepackage{{setspace}}
\usepackage{{verse}}
\usepackage[x11names]{{xcolor}}
\usepackage{{aurical}}
\usepackage[default]{{frcursive}}

\begin{{document}}
\begin{{center}}
{{
    \Fontskrivan\bfseries\slshape
    \fontsize{{40pt}}{{35pt}}\selectfont
    \color{{blue}}
    {}
}} \\[0.5em]
{{
    \textcursive{{
        \bfseries
        \fontsize{{16pt}}{{12pt}}\selectfont
        \color{{orange}}
        {}
    }}
}}
\end{{center}}
\vspace{{1em}}

{}
{}
\end{{document}}
"#,
            song.info.title, song.info.author, settings.lyrics_font, lyrics_inputs
        );

        if let Err(e) = std::fs::write(&lyrics_tex_full_path, &lyrics_tex_content) {
            log::error!("Failed to write lyrics/main.tex: {e}");
            return Err(ExpandError::Other(e.to_string()));
        }

        // Create nodes (body.tex and lyrics files are added as root nodes in make_all)
        let tex_node = TexFile::new(tex_path.clone());
        let tikz_node = SongTikz::new(tikz_path.clone());
        let preamble_node = TexFile::new(preamble_path);
        let spline_node = TexFile::new(spline_path);
        let sections_node = TexFile::new(sections_path);
        let chords_node = TexFile::new(chords_path);
        let data_node = TexFile::new(data_path);
        let macros_ly_node = LilypondFile::new(macros_ly_path);
        let lyrics_tex_node = TexFile::new(lyrics_tex_path.clone());

        // Scan for lilypond files referenced in tex files
        let pdf_for_scan = PdfFile::new(pdf_path.clone());
        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![&tex_node];
        let (_, _scanned_inputs, toplevel_ly) =
            pdf_for_scan.scan_with_toplevel_ly(sandbox, &predecessors);

        // Only use top-level .ly files (from \lyfile{} or \songly{}) for LyTexFile nodes
        // Included .ly files (from \include) don't need LyTexFile/TexOfLilypond
        let ly_files = toplevel_ly;

        // Create lyrics PDF path
        let lyrics_pdf_path = parent_dir.join("lyrics").join("main.pdf");
        let lyrics_pdf_node = PdfFile::new(lyrics_pdf_path.clone());

        let mut nodes: Vec<Box<dyn GNode + Send + Sync>> = vec![
            Box::new(tex_node),
            Box::new(tikz_node),
            Box::new(preamble_node),
            Box::new(spline_node),
            Box::new(sections_node),
            Box::new(chords_node),
            Box::new(data_node),
            Box::new(macros_ly_node),
            Box::new(lyrics_tex_node),
            Box::new(lyrics_pdf_node),
        ];

        let mut edges: Vec<Edge> = vec![];

        // NOTE: PdfFile must be pre-added to the graph with Initial status
        // before calling make(), otherwise yamake marks expanded nodes as
        // "mounted" which skips the build phase
        let main_edge = Edge {
            nfrom: Box::new(TexFile::new(tex_path)),
            nto: Box::new(PdfFile::new(pdf_path.clone())),
        };
        edges.push(main_edge);

        // Edge: lyrics/main.tex -> lyrics/main.pdf
        let lyrics_edge = Edge {
            nfrom: Box::new(TexFile::new(lyrics_tex_path)),
            nto: Box::new(PdfFile::new(lyrics_pdf_path)),
        };
        edges.push(lyrics_edge);

        // Add LilypondFile -> LyTexFile -> PdfFile chain
        // Add LilypondFile -> TexOfLilypond -> PdfFile chain
        for ly_path in ly_files {
            // Get stem (e.g., "interlude" from "parent/interlude.ly")
            let stem = ly_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            let ly_parent = ly_path.parent().unwrap_or(Path::new(""));

            // Create LyTexFile path (same as ly but with .lytex extension)
            let lytex_path = ly_path.with_extension("lytex");

            // Create TexOfLilypond path (e.g., parent/interlude.output/interlude.tex)
            let texoflypath = ly_parent
                .join(format!("{stem}.output"))
                .join(format!("{stem}.tex"));

            let ly_node = LilypondFile::new(ly_path.clone());
            let lytex_node = LyTexFile::new(lytex_path.clone());
            let texofly_node = TexOfLilypond::new(texoflypath.clone());
            nodes.push(Box::new(ly_node));
            nodes.push(Box::new(lytex_node));
            nodes.push(Box::new(texofly_node));

            // Edge: LilypondFile -> LyTexFile
            let ly_to_lytex_edge = Edge {
                nfrom: Box::new(LilypondFile::new(ly_path.clone())),
                nto: Box::new(LyTexFile::new(lytex_path.clone())),
            };
            edges.push(ly_to_lytex_edge);

            // Edge: LyTexFile -> PdfFile
            let lytex_to_pdf_edge = Edge {
                nfrom: Box::new(LyTexFile::new(lytex_path.clone())),
                nto: Box::new(PdfFile::new(pdf_path.clone())),
            };
            edges.push(lytex_to_pdf_edge);

            // Edge: LilypondFile -> TexOfLilypond
            let ly_to_texofly_edge = Edge {
                nfrom: Box::new(LilypondFile::new(ly_path)),
                nto: Box::new(TexOfLilypond::new(texoflypath.clone())),
            };
            edges.push(ly_to_texofly_edge);

            // Edge: LyTexFile -> TexOfLilypond
            let lytex_to_texofly_edge = Edge {
                nfrom: Box::new(LyTexFile::new(lytex_path)),
                nto: Box::new(TexOfLilypond::new(texoflypath.clone())),
            };
            edges.push(lytex_to_texofly_edge);

            // Edge: TexOfLilypond -> PdfFile
            let texofly_to_pdf_edge = Edge {
                nfrom: Box::new(TexOfLilypond::new(texoflypath)),
                nto: Box::new(PdfFile::new(pdf_path.clone())),
            };
            edges.push(texofly_to_pdf_edge);
        }

        Ok((nodes, edges))
    }
}
