# band-songbook

A build system for generating PDF songbooks with chord charts and LilyPond music notation.

## Features

- Parses `song.yml` files defining song structure (chords, lyrics, sections)
- Generates LaTeX/TikZ chord charts
- Integrates LilyPond for music notation (tablature, scores)
- Incremental builds using the yamake dependency graph system
- Fuzzy pattern matching to build specific songs

## Installation

```bash
cargo install band-songbook
```

### Requirements

- **LuaLaTeX** - for PDF generation
- **LilyPond** with `lilypond-book` - for music notation (optional)

## Usage

```bash
band-songbook --srcdir <SOURCE_DIR> --sandbox <OUTPUT_DIR> [--settings <SETTINGS_FILE>] [--pattern <PATTERN>]
```

### Options

- `-s, --srcdir` - Source directory containing `song.yml` files
- `-o, --sandbox` - Output directory for generated files
- `-c, --settings` - Path to `settings.yml` for colors and configuration
- `-p, --pattern` - Fuzzy pattern to filter songs (e.g., "beatles" or "yesterday")

### Example

```bash
band-songbook --srcdir ./songs --sandbox ./build --settings ./settings.yml
```

Build only songs matching "velvet":
```bash
band-songbook --srcdir ./songs --sandbox ./build --pattern velvet
```

## Song Format

Each song is defined by a `song.yml` file:

```yaml
info:
  author: "Artist Name"
  title: "Song Title"
  tempo: 120

structure:
  - id: intro
    item:
      Chords:
        section_type: intro
        chords: "Am | G | F | E"

  - id: verse1
    item:
      Chords:
        section_type: couplet
        chords: "Am | G | C | F | Am | G | E | E"

  - id: chorus
    item:
      Chords:
        section_type: refrain
        chords: "F | G | Am | Am | F | G | C | C"
```

### Directory Structure

```
songs/
  artist_name/
    song_title/
      song.yml        # Song definition
      body.tex        # Main content template
      lyrics/
        intro.tex     # Lyrics for intro section
        verse1.tex    # Lyrics for verse1 section
        chorus.tex    # Lyrics for chorus section
      interlude.ly    # Optional LilyPond notation
```

## License

MIT
