# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.33]

### Fixed

- A negative click time lost its sign when the minutes were zero: `-00:02.00`
  parsed as `+2.0` seconds, because the minutes field was parsed on its own and
  `-00` is just `-0`. The sign is now taken from the whole string before
  splitting, so `-00:02.00` is -2 s. A sign inside the value (`00:-02.00`) is
  rejected with a message pointing at the `-mm:ss.ms` form, and surrounding
  whitespace is trimmed.

- `clicks.yml` did not depend on `song.yml` in the build graph, so a
  `section_id` anchor kept its old bar number after the section it points at
  changed length. The edge is now declared and the clicks are re-pinned on
  rebuild.

### Removed

- `go.sh`, `start-aws.sh` and `start.json`: local dev helpers that still
  referred to the `software/` directory and the `legendary-memory` repository,
  neither of which exists.

## [0.0.32]

### Added

- `.mp3` renders for sections listed under `files.mp3` in `song.yml` (the field
  was previously `files.wav` and unused). A new `MidiOfLilypond` node runs
  `lilypond` directly on `<stem>.ly` to get a predictable `<stem>.midi`, and a
  new `Mp3Render` node synthesises it with `fluidsynth` and encodes it with
  `ffmpeg`. The soundfont is taken from `soundfont:` in `settings.yml`, then
  `BAND_SONGBOOK_SOUNDFONT`, then the usual system paths. Declaring a render
  does not put the score in the PDF. Renders are delivered under
  `mp3-renders/<author>--@--<title>-<section>.mp3`.

- A standalone cropped PDF per LilyPond file, so a single score can be shown on
  its own. A new `PdfOfLilypond` node runs `lilypond -dcrop`, and the result is
  delivered under `pdf-snippets/<author>--@--<title>-<section>.pdf`.

- `clicks-def.yml` entries can reference a section instead of a bar:
  `section_id: couplet1` resolves to that section's first bar, so a click
  definition survives edits to the sections above it. `bar_number` and
  `section_id` are mutually exclusive, and exactly one of them is required.

- `\mypull`, `\mypulled` and `\myrelease` in `macros.ly`, for guitar
  articulation marks.

### Fixed

- Accented letters in a song's author or title no longer become `_` in delivered
  file names: they fold to their ASCII base, so "Noir Desir / Marlene" is
  `noir_desir--@--marlene` and not `noir_d_sir--@--marl_ne`. This renames
  existing delivery artifacts for songs with accents.

- `files.lilypond` in `song.yml` was parsed but never used. Declared `.ly` files
  are now build dependencies, so editing one rebuilds the song PDF even when no
  tex file references it. A declared file that does not exist is skipped with a
  warning.

- The chord chart printed a hardcoded `140 BPM` instead of the song's own
  `info.tempo`.

### Changed

- `\basecouplet` no longer reserves a line of vertical space for a section whose
  lyrics file is empty, so a run of lyric-less sections stacks tightly

- `clicks-def.yml` is rejected when its times are not strictly increasing. The
  click times are interpolated from the gap between consecutive entries, so
  equal or backwards times used to produce garbage silently.

- The `files.wav` field in `song.yml` is now `files.mp3`. Nothing ever read
  `files.wav`, so this only matters for song.yml files that set it.

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

[Unreleased]: https://github.com/laurentcarrie/songbook-lambda/compare/band-songbook-v0.0.33...HEAD
[0.0.33]: https://github.com/laurentcarrie/songbook-lambda/releases/tag/band-songbook-v0.0.33
[0.0.32]: https://github.com/laurentcarrie/songbook-lambda/releases/tag/band-songbook-v0.0.32
[0.0.1]: https://github.com/laurentcarrie/songbook-lambda/releases/tag/band-songbook-v0.0.1
