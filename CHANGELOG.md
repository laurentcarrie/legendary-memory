# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1] - 2026-02-03

### Added

- Initial release
- Song discovery from `song.yml` files
- Chord parsing with support for major, minor, 7th, dim, sus2, sus4 chords
- Sharp and flat accidentals
- Rest notation (HRest)
- Repeat markers (x2, x3, etc.)
- LaTeX/TikZ chord chart generation
- LilyPond integration via `lilypond-book`
- Incremental builds using yamake dependency graph
- Fuzzy pattern matching for selective builds
- Configurable section colors via `settings.yml`
- Handlebars templating for LaTeX output
- Support for lyrics files per section
- Mermaid graph output for build visualization

### Dependencies

- yamake v0.1.9 for build system
- handlebars for templating
- serde/serde_yaml for configuration parsing
- argh for CLI argument parsing

[Unreleased]: https://github.com/music-practice-tools/legendary-memory/compare/band-songbook-v0.0.1...HEAD
[0.0.1]: https://github.com/music-practice-tools/legendary-memory/releases/tag/band-songbook-v0.0.1
