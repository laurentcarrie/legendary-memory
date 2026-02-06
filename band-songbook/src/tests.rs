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
        let song: Song = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");

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
        assert!(
            mounted_path.exists(),
            "song.yml should be mounted in sandbox"
        );

        // Verify main.tex was created by expand
        let tex_path = sandbox.path().join("PJHarvey/Dress/main.tex");
        assert!(tex_path.exists(), "main.tex should be created in sandbox");

        // Verify main.pdf was built
        let pdf_path = sandbox.path().join("PJHarvey/Dress/main.pdf");
        assert!(pdf_path.exists(), "main.pdf should be built in sandbox");
    }

    #[test]
    fn test_yamake_build_song_with_lilypond() {
        let srcdir = PathBuf::from("tests/data");
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        let mut g = G::new(srcdir, sandbox.path().to_path_buf());

        // Add settings.yml as root node so it gets copied to sandbox
        let colors_node = TexFile::new(PathBuf::from("settings.yml"));
        let _ = g.add_root_node(colors_node);

        // mademoiselle_K/ca_me_vexe - has lilypond files
        let song_node = SongYml::new(PathBuf::from("mademoiselle_K/ca_me_vexe/song.yml"));
        let song_idx = g.add_root_node(song_node).expect("Failed to add song node");

        let body_node = TexFile::new(PathBuf::from("mademoiselle_K/ca_me_vexe/body.tex"));
        let _ = g.add_root_node(body_node);

        // Add lyrics files as root nodes (like make_all does)
        let lyrics_files = [
            "intro",
            "couplet1",
            "refrain1",
            "couplet2a",
            "couplet2b",
            "couplet2c",
            "refrain2",
            "couplet3a",
            "couplet3b",
            "refrain3",
            "interlude",
            "final",
        ];
        for name in lyrics_files {
            let lyrics_path = PathBuf::from(format!("mademoiselle_K/ca_me_vexe/lyrics/{name}.tex"));
            let lyrics_node = TexFile::new(lyrics_path);
            let _ = g.add_root_node(lyrics_node);
        }

        let pdf_node = PdfFile::new(PathBuf::from("mademoiselle_K/ca_me_vexe/main.pdf"));
        let pdf_idx = g.add_node(pdf_node).expect("Failed to add pdf node");
        g.add_edge(song_idx, pdf_idx);

        let success = g.make();
        assert!(success, "Build should succeed");
        // Verify main.pdf was built
        let pdf_path = sandbox.path().join("mademoiselle_K/ca_me_vexe/main.pdf");
        assert!(
            pdf_path.exists(),
            "mademoiselle_K/ca_me_vexe/main.pdf should be built in sandbox"
        );

        // Verify interlude.ly node exists with tag 'lilypond'
        let interlude_ly_found = g.g.node_indices().any(|idx| {
            let node = &g.g[idx];
            node.pathbuf() == PathBuf::from("mademoiselle_K/ca_me_vexe/interlude.ly")
                && node.tag() == "lilypond"
        });
        assert!(
            interlude_ly_found,
            "interlude.ly node should exist with tag 'lilypond'"
        );

        // Verify interlude.ly is in the predecessor tree of main.pdf
        let predecessors = g.root_predecessors(pdf_idx);
        let interlude_is_predecessor = predecessors.iter().any(|&idx| {
            g.g[idx].pathbuf() == PathBuf::from("mademoiselle_K/ca_me_vexe/interlude.ly")
        });
        assert!(
            interlude_is_predecessor,
            "interlude.ly should be a predecessor of main.pdf"
        );

        // Verify interlude.output/interlude.tex was generated by lilypond-book
        let interlude_tex_path = sandbox
            .path()
            .join("mademoiselle_K/ca_me_vexe/interlude.output/interlude.tex");
        assert!(
            interlude_tex_path.exists(),
            "interlude.output/interlude.tex should be generated"
        );

        // Verify macros.ly is in the predecessor tree of main.pdf (via interlude.ly \include)
        let macros_is_predecessor = predecessors
            .iter()
            .any(|&idx| g.g[idx].pathbuf() == PathBuf::from("mademoiselle_K/ca_me_vexe/macros.ly"));
        assert!(
            macros_is_predecessor,
            "macros.ly should be a predecessor of main.pdf"
        );
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

        let (success, _g) = make_all(
            srcdir,
            sandbox.path(),
            Some(Path::new("tests/data/settings.yml")),
            None,
        );
        assert!(success, "make_all should succeed");

        // Verify PDFs were created for both songs
        let pdf1 = sandbox.path().join("songs/PJHarvey/Dress/main.pdf");
        assert!(pdf1.exists(), "songs/PJHarvey/Dress/main.pdf should be created");

        let pdf2 = sandbox.path().join("songs/mademoiselle_K/ca_me_vexe/main.pdf");
        assert!(
            pdf2.exists(),
            "songs/mademoiselle_K/ca_me_vexe/main.pdf should be created"
        );
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
    fn test_make_all_with_pattern() {
        let srcdir = Path::new("tests/data");
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        // Pattern "madkvex" should match "Mademoiselle K Ca me vexe"
        let (success, _g) = make_all(
            srcdir,
            sandbox.path(),
            Some(Path::new("tests/data/settings.yml")),
            Some("madkvex"),
        );
        assert!(success, "make_all with pattern should succeed");

        // Only Ca Me Vexe should be built (matches "Mademoiselle K Ca me vexe")
        let pdf_vexe = sandbox.path().join("songs/mademoiselle_K/ca_me_vexe/main.pdf");
        assert!(
            pdf_vexe.exists(),
            "songs/mademoiselle_K/ca_me_vexe/main.pdf should be created"
        );

        // PJHarvey/Dress should NOT be built (doesn't match pattern)
        let pdf_dress = sandbox.path().join("songs/PJHarvey/Dress/main.pdf");
        assert!(
            !pdf_dress.exists(),
            "songs/PJHarvey/Dress/main.pdf should NOT be created"
        );
    }

    #[test]
    fn test_parse_invalid_chord() {
        use crate::chords::parse::ParseError;

        let invalid_inputs = ["X", "Am+", "Ax"];
        for input in invalid_inputs {
            let result = parse(input);
            assert!(result.is_err(), "Expected error for input: {input}");
            assert_eq!(
                result.unwrap_err(),
                ParseError::InvalidChord(input.to_string())
            );
        }
    }

    #[tokio::test]
    async fn test_make_all_with_storage_local() {
        use crate::make_all_with_storage;

        let srcdir = "tests/data";
        let sandbox = tempfile::tempdir().expect("Failed to create temp dir");
        let sandbox_path = sandbox.path().to_str().unwrap();
        let settings = "tests/data/settings.yml";

        let result =
            make_all_with_storage(srcdir, sandbox_path, sandbox.path(), Some(settings), None).await;

        assert!(result.is_ok(), "make_all_with_storage should succeed");
        let (success, _g) = result.unwrap();
        assert!(success, "Build should succeed");

        // Verify PDFs were created
        let pdf1 = sandbox.path().join("songs/PJHarvey/Dress/main.pdf");
        assert!(pdf1.exists(), "songs/PJHarvey/Dress/main.pdf should be created");

        let pdf2 = sandbox.path().join("songs/mademoiselle_K/ca_me_vexe/main.pdf");
        assert!(
            pdf2.exists(),
            "songs/mademoiselle_K/ca_me_vexe/main.pdf should be created"
        );
    }

    /// Integration test for S3 storage.
    /// Run with: AWS_PROFILE=zik-laurent cargo test test_make_all_with_s3 -- --ignored --nocapture
    #[tokio::test]
    #[ignore]
    async fn test_make_all_with_s3() {
        use crate::make_all_with_storage;

        let srcdir = "s3://zik-laurent/songs";
        let sandbox = "s3://zik-laurent/output";
        let settings = "s3://zik-laurent/songs/settings.yml";
        let local_sandbox = tempfile::tempdir().expect("Failed to create temp dir");

        let result =
            make_all_with_storage(srcdir, sandbox, local_sandbox.path(), Some(settings), None)
                .await;

        match &result {
            Ok((success, _g)) => {
                println!("Build completed, success: {success}");
                assert!(*success, "Build should succeed");
            }
            Err(e) => {
                panic!("make_all_with_storage failed: {e}");
            }
        }
    }
}
