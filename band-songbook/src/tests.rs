#[cfg(test)]
mod tests {
    use crate::chords::parse::parse;
    use crate::discover;
    use crate::make_all;
    use crate::model::Song;
    use crate::nodes::{PdfFile, SongYml, TexFile};
    use std::path::{Path, PathBuf};
    use yamake::model::{G, GNode};

    #[test]
    fn test_read_song_from_yaml() {
        let yaml_content = std::fs::read_to_string("tests/data/PJHarvey/Dress/song.yml")
            .expect("Failed to read song.yml");
        let song: Song = serde_yaml::from_str(&yaml_content)
            .expect("Failed to parse YAML");

        assert_eq!(song.info.author, "P.J. Harvey");
        assert_eq!(song.info.title, "Dress");
        assert_eq!(song.info.tempo, 96);
    }

    #[test]
    fn test_yamake_build_song() {
        let srcdir = PathBuf::from("tests/data");
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        let mut g = G::new(srcdir, sandbox.path().to_path_buf());

        // Add settings.yml as root node so it gets copied to sandbox
        let colors_node = TexFile::new(PathBuf::from("settings.yml"));
        let _ = g.add_root_node(colors_node);

        // SongYml is a root node (source file)
        let song_node = SongYml::new(PathBuf::from("PJHarvey/Dress/song.yml"));
        let song_idx = g.add_root_node(song_node).expect("Failed to add song node");

        // body.tex is a root node (source file from srcdir)
        let body_node = TexFile::new(PathBuf::from("PJHarvey/Dress/body.tex"));
        let _ = g.add_root_node(body_node);

        // Pre-add PdfFile with Initial status so it goes through build loop
        // (yamake marks expanded nodes as "mounted" which skips build)
        // Add edge from SongYml so PdfFile isn't treated as a root node
        let pdf_node = PdfFile::new(PathBuf::from("PJHarvey/Dress/main.pdf"));
        let pdf_idx = g.add_node(pdf_node).expect("Failed to add pdf node");
        g.add_edge(song_idx, pdf_idx);

        let success = g.make();
        assert!(success, "Build should succeed");

        // Verify the file was mounted to sandbox
        let mounted_path = sandbox.path().join("PJHarvey/Dress/song.yml");
        assert!(mounted_path.exists(), "song.yml should be mounted in sandbox");

        // Verify main.tex was created by expand
        let tex_path = sandbox.path().join("PJHarvey/Dress/main.tex");
        assert!(tex_path.exists(), "main.tex should be created in sandbox");

        // Verify main.pdf was built
        let pdf_path = sandbox.path().join("PJHarvey/Dress/main.pdf");
        assert!(pdf_path.exists(), "main.pdf should be built in sandbox");
    }

    #[test]
    fn test_yamake_build_pdf() {
        let srcdir = PathBuf::from("tests/data");
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        let mut g = G::new(srcdir, sandbox.path().to_path_buf());

        // Add TexFile as root node
        let tex_node = TexFile::new(PathBuf::from("tex/hello.tex"));
        let tex_idx = g.add_root_node(tex_node).expect("Failed to add tex node");

        // Add PdfFile as build node
        let pdf_node = PdfFile::new(PathBuf::from("tex/hello.pdf"));
        let pdf_idx = g.add_node(pdf_node).expect("Failed to add pdf node");

        // Add edge: tex -> pdf
        g.add_edge(tex_idx, pdf_idx);

        let success = g.make();
        assert!(success, "Build should succeed");

        // Verify the PDF was created
        let pdf_path = sandbox.path().join("tex/hello.pdf");
        assert!(pdf_path.exists(), "hello.pdf should be created in sandbox");
    }

    #[test]
    fn test_discover() {
        let mut songs = discover(Path::new("tests/data"));
        songs.sort();

        assert_eq!(songs.len(), 2);
        assert!(songs[0].ends_with("PJHarvey/Dress/song.yml"));
        assert!(songs[1].ends_with("mademoiselle_K/ca_me_vexe/song.yml"));
    }

    #[test]
    fn test_make_all() {
        let srcdir = Path::new("tests/data");
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        let (success, _g) = make_all(srcdir, sandbox.path());
        assert!(success, "make_all should succeed");

        // Verify PDFs were created for both songs
        let pdf1 = sandbox.path().join("PJHarvey/Dress/main.pdf");
        assert!(pdf1.exists(), "PJHarvey/Dress/main.pdf should be created");

        let pdf2 = sandbox.path().join("mademoiselle_K/ca_me_vexe/main.pdf");
        assert!(pdf2.exists(), "mademoiselle_K/ca_me_vexe/main.pdf should be created");
    }

    #[test]
    fn test_pdf_scan() {
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        // Create a tex file with \input instructions
        let tex_dir = sandbox.path().join("song");
        std::fs::create_dir_all(&tex_dir).expect("Failed to create dir");

        let tex_content = r#"\documentclass{article}
\begin{document}
\input{intro.tex}
\input{verse1.tex}
\input{chorus.tex}
\end{document}
"#;
        std::fs::write(tex_dir.join("main.tex"), tex_content).expect("Failed to write tex file");

        let tex_node = TexFile::new(PathBuf::from("song/main.tex"));
        let pdf_node = PdfFile::new(PathBuf::from("song/main.pdf"));

        let predecessors: Vec<&(dyn GNode + Send + Sync)> = vec![&tex_node];
        let (success, inputs) = pdf_node.scan(sandbox.path(), &predecessors);

        assert!(success);
        assert_eq!(inputs.len(), 3);
        assert_eq!(inputs[0], PathBuf::from("song/intro.tex"));
        assert_eq!(inputs[1], PathBuf::from("song/verse1.tex"));
        assert_eq!(inputs[2], PathBuf::from("song/chorus.tex"));
    }

    #[test]
    fn test_parse_chords() {
        use crate::chords::model::{Alteration, BarItem, Repeat, Rest};

        let input = "Em | Em|C7|G|Am Bm7|HRest|Csm7|Edim|Bsus4|Cssus2|x2";
        let result = parse(input).unwrap();
        assert_eq!(result.bars.len(), 10);
        assert_eq!(result.repeat, Repeat { n: 2 });

        // First bar: Em (E minor)
        assert_eq!(result.bars[0].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[0].items[0] {
            assert_eq!(chord.name, "E");
            assert!(chord.minor);
            assert_eq!(chord.alteration, Alteration::None);
        } else {
            panic!("Expected Chord");
        }

        // Second bar: Em (E minor)
        assert_eq!(result.bars[1].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[1].items[0] {
            assert_eq!(chord.name, "E");
            assert!(chord.minor);
        } else {
            panic!("Expected Chord");
        }

        // Third bar: C7 (C seventh)
        assert_eq!(result.bars[2].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[2].items[0] {
            assert_eq!(chord.name, "C");
            assert!(!chord.minor);
            assert_eq!(chord.alteration, Alteration::Seven);
        } else {
            panic!("Expected Chord");
        }

        // Fourth bar: G (G major)
        assert_eq!(result.bars[3].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[3].items[0] {
            assert_eq!(chord.name, "G");
            assert!(!chord.minor);
            assert_eq!(chord.alteration, Alteration::None);
        } else {
            panic!("Expected Chord");
        }

        // Fifth bar: Am Bm7 (two chords)
        assert_eq!(result.bars[4].items.len(), 2);
        // Am (A minor)
        if let BarItem::Chord(chord) = &result.bars[4].items[0] {
            assert_eq!(chord.name, "A");
            assert!(chord.minor);
            assert_eq!(chord.alteration, Alteration::None);
        } else {
            panic!("Expected Chord");
        }
        // Bm7 (B minor seventh)
        if let BarItem::Chord(chord) = &result.bars[4].items[1] {
            assert_eq!(chord.name, "B");
            assert!(chord.minor);
            assert_eq!(chord.alteration, Alteration::Seven);
        } else {
            panic!("Expected Chord");
        }

        // Sixth bar: HRest
        assert_eq!(result.bars[5].items.len(), 1);
        assert_eq!(result.bars[5].items[0], BarItem::Rest(Rest { duration: 1 }));

        // Seventh bar: Csm7 (C sharp minor seventh)
        assert_eq!(result.bars[6].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[6].items[0] {
            assert_eq!(chord.name, "C");
            assert_eq!(chord.accidental, crate::chords::model::Accidental::Sharp);
            assert!(chord.minor);
            assert_eq!(chord.alteration, Alteration::Seven);
        } else {
            panic!("Expected Chord");
        }

        // Eighth bar: Edim (E diminished)
        assert_eq!(result.bars[7].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[7].items[0] {
            assert_eq!(chord.name, "E");
            assert!(!chord.minor);
            assert_eq!(chord.alteration, Alteration::Dim);
        } else {
            panic!("Expected Chord");
        }

        // Ninth bar: Bsus4 (B sus4)
        assert_eq!(result.bars[8].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[8].items[0] {
            assert_eq!(chord.name, "B");
            assert!(!chord.minor);
            assert_eq!(chord.alteration, Alteration::Sus4);
        } else {
            panic!("Expected Chord");
        }

        // Tenth bar: Cssus2 (C sharp sus2)
        assert_eq!(result.bars[9].items.len(), 1);
        if let BarItem::Chord(chord) = &result.bars[9].items[0] {
            assert_eq!(chord.name, "C");
            assert_eq!(chord.accidental, crate::chords::model::Accidental::Sharp);
            assert!(!chord.minor);
            assert_eq!(chord.alteration, Alteration::Sus2);
        } else {
            panic!("Expected Chord");
        }
    }

    #[test]
    fn test_parse_invalid_chord() {
        use crate::chords::parse::ParseError;

        let invalid_inputs = ["X", "Am+", "Ax"];
        for input in invalid_inputs {
            let result = parse(input);
            assert!(result.is_err(), "Expected error for input: {}", input);
            assert_eq!(result.unwrap_err(), ParseError::InvalidChord(input.to_string()));
        }
    }
}
