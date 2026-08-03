# Changelog

All notable changes to this project will be documented in this file.

## [0.0.33] - 2026-08-03

### Fixed
- A negative click time lost its sign when the minutes were zero: `-00:02.00` parsed as `+2.0` seconds, because the minutes field was parsed on its own and `-00` is just `-0`. The sign is now taken from the whole string before splitting, so `-00:02.00` is -2 s. A sign inside the value (`00:-02.00`) is rejected with a message pointing at the `-mm:ss.ms` form, and surrounding whitespace is trimmed
- `clicks.yml` did not depend on `song.yml` in the build graph, so a `section_id` anchor kept its old bar number after the section it points at changed length. The edge is now declared and the clicks are re-pinned on rebuild

### Removed
- `go.sh`, `start-aws.sh` and `start.json`: local dev helpers that still referred to the `software/` directory and the `legendary-memory` repository, neither of which exists

## [0.0.32] - 2026-08-03

### Added
- `.mp3` renders for sections listed under `files.mp3` in `song.yml` (the field was previously `files.wav` and unused): a new `MidiOfLilypond` node runs `lilypond` directly on `<stem>.ly` to get a predictable `<stem>.midi`, and a new `Mp3Render` node synthesises it with `fluidsynth` and encodes it with `ffmpeg`. The soundfont comes from `soundfont:` in `settings.yml`, then `BAND_SONGBOOK_SOUNDFONT`, then the usual system paths. Declaring a render does not put the score in the PDF. Delivered under `mp3-renders/<author>--@--<title>-<section>.mp3`
- A standalone cropped PDF per LilyPond file, so a single score can be shown on its own: a new `PdfOfLilypond` node runs `lilypond -dcrop`, delivered under `pdf-snippets/<author>--@--<title>-<section>.pdf`
- `clicks-def.yml` entries can reference a section instead of a bar: `section_id: couplet1` resolves to that section's first bar, so a click definition survives edits to the sections above it. `bar_number` and `section_id` are mutually exclusive, and exactly one of them is required
- `\mypull`, `\mypulled` and `\myrelease` in `macros.ly`, for guitar articulation marks

### Fixed
- Accented letters in a song's author or title no longer become `_` in delivered file names: they fold to their ASCII base, so "Noir Desir / Marlene" is `noir_desir--@--marlene` and not `noir_d_sir--@--marl_ne`. This renames existing delivery artifacts for songs with accents
- `files.lilypond` in `song.yml` was parsed but never used: declared `.ly` files are now build dependencies, so editing one rebuilds the song PDF even when no tex file references it. A declared file that does not exist is skipped with a warning
- The chord chart printed a hardcoded `140 BPM` instead of the song's own `info.tempo`

### Changed
- `\basecouplet` no longer reserves a line of vertical space for a section whose lyrics file is empty, so a run of lyric-less sections stacks tightly
- `clicks-def.yml` is rejected when its times are not strictly increasing: the click times are interpolated from the gap between consecutive entries, so equal or backwards times used to produce garbage silently
- The `files.wav` field in `song.yml` is now `files.mp3`. Nothing ever read `files.wav`, so this only matters for `song.yml` files that set it

## [0.0.31] - 2026-07-29

### Fixed
- Song title and author now appear in the PDF footer: `data.tex` read `{{{title}}}`/`{{{author}}}`/`{{{tempo}}}`, but the handlebars context nests these under `song.info`, so `\xxcfoot` was fed empty strings
- The footer's `dernière modif le` date now comes from `song.meta.date` instead of the non-existent `{{date}}`, and the whole clause is omitted when a song has no date

### Changed
- `meta` and its `date`/`digest` fields are now `#[serde(default)]`, so a `song.yml` with a partial or missing `meta:` block still parses

## [0.0.20] - 2026-02-26

### Added
- Click track analysis node (`ClickYml`) that decodes an MP3, detects ticks via amplitude threshold, and outputs tick offsets as YAML (`clicks.yml`)
- `Mp3` root node for MP3 audio files
- `clicks` optional field in `song.yml` files section to specify a click track MP3 (e.g. `clicks: clicks.mp3`)
- `minimp3` dependency for pure-Rust MP3 decoding
- Integration test for full build graph with click track

## [0.0.11] - 2026-02-08

### Added
- Two lyrics PDF variants per song: 1-column and 2-column layouts
  - Delivered to `pdf-lyrics-1-column/` and `pdf-lyrics-2-column/` directories
- Bar number ranges displayed in lyrics section headers (e.g., "couplet 1 (9 -> 16)")
- `settings.lyrics_font` now uses `\setmainfont` (fontspec) instead of `\fontfamily` (NFSS) for proper lualatex compatibility

### Changed
- `\basecouplet` title box now spans full column width
- `\basecouplet` content starts on a new line below the title
- Lyrics PDFs use `\basecouplet` macro (same as main PDF) for consistent font rendering
- 1-column lyrics layout is centered; 2-column is left-aligned

### Removed
- Single `pdf-lyrics/` delivery directory (replaced by two variant directories)

## [0.0.10] - 2026-02-08

### Changed
- Replace Times New Roman with TeX Gyre Termes in lyrics PDF template (Lambda compatible)
- Publish workflow now triggers automatically on push to `main` (replaces tag-based trigger)
- Publish skips if the version is already on crates.io
- Bump `Cargo.toml` version to match changelog; fix repository URL

### Removed
- Stale `SRCDIR`, `SANDBOX`, `SETTINGS` env vars from Dockerfile
- Obsolete `deploy-lambda.yml` workflow (superseded by `deploy-lambda-docker.yml`)

## [0.0.9] - 2026-02-07

### Added
- Mandatory `--delivery` / `-d` CLI argument for specifying where final PDF files are copied
- Delivery supports both local paths and S3 paths
- Lambda now receives `srcdir`, `settings`, and `delivery` via invocation payload instead of environment variables

### Changed
- Sandbox is now always a local path (removed S3 sandbox support)
- `--settings` / `-c` CLI argument is now mandatory
- `make_all_with_storage` signature simplified: `sandbox` is now `&Path`, removed `local_sandbox` parameter
- Lambda deployment only sets `RUST_LOG` env var; all other config is per-invocation

### Removed
- S3 sandbox upload logic
- `deploy-lambda.sh` and `deploy-docker-lambda.sh` scripts (superseded by GitHub Actions workflow)
- Default values for `SRCDIR`, `SETTINGS`, and `DELIVERY` in Lambda

## [0.0.8] - 2026-02-06

### Added
- Bar count display in TikZ song diagrams
  - Shows bar number at the left of each row (Chords and Ref sections)
  - Tracks cumulative bar count including repeats
- Row multiplier display (xN) at end of rows when repeat > 1
- `row_multiplier` Handlebars helper to extract repeat count from chord rows
- `ref_bar_count` Handlebars helper to calculate total bars for referenced sections

### Changed
- Section labels moved slightly more to the left for better layout

## [0.0.7] - 2026-02-05

### Changed
- `make_all_with_storage` now takes `local_sandbox` as explicit parameter
- Lambda uses fixed `/tmp/sandbox` path instead of temp directory

### Improved
- lilypond-book errors now log stdout and stderr content for easier debugging

## [0.0.6] - 2026-02-04

### Fixed
- Lambda now shows application logs in CloudWatch (added `tracing-log` feature)

## [0.0.5] - 2026-02-04

### Changed
- CLI now supports S3 paths for srcdir, sandbox, and settings arguments
- Main function is now async using tokio runtime
- Uses `make_all_with_storage` for unified local/S3 path handling

## [0.0.4] - 2026-02-04

### Added
- GitHub Actions workflow for Lambda deployment (`deploy-lambda.yml`)
  - Accepts srcdir, sandbox, settings, and pattern as inputs
  - Builds with cargo-lambda for AWS Lambda
  - Deploys and optionally invokes the function

### Fixed
- Log files are now correctly uploaded to S3 (recursive directory traversal)

## [0.0.3] - 2026-02-04

### Added
- AWS Lambda function binary (`band-songbook-lambda`) for serverless builds
- Lambda accepts S3 paths for srcdir, sandbox, settings, and pattern
- S3 integration test (`test_make_all_with_s3`)

## [0.0.2] - 2026-02-04

### Added
- `get_lilypond_files` public function to extract LilyPond file dependencies from a PdfFile node
- `PdfFile` re-exported from crate root for easier access
- Pre-build check for `lualatex` availability with clear error message
- Log files are now uploaded to S3 along with PDF files
- `todo.md` for tracking future work

### Fixed
- Repository URL in Cargo.toml (now points to correct GitHub repository)

## [0.0.1] - Initial release

### Added
- Core build system for generating PDF songbooks
- Support for chord charts and LilyPond music notation
- S3 storage support for source and output files
- Song discovery and pattern-based filtering
