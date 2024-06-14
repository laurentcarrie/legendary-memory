\version "2.20.0"

song_tempo = 114


\paper {
  #(include-special-characters)
  indent = 0\mm
  line-width = 180\mm
  oddHeaderMarkup = ""
  evenHeaderMarkup = ""
  oddFooterMarkup = ""
  evenFooterMarkup = ""

  #(add-text-replacements!
    '(
       ("100" . "hundred")
       ("dpi" . "dots per inch")
       ))

}


\score {
  <<

    \new ChordNames \with {
      \override BarLine.bar-extent = #'(-2 . 2)
      \consists "Bar_engraver"
    }

    \chordmode {
      e1 | e1 | e1 | e1
    }

\new ChordGrid \chordmode { c1 r2 c2 R1 }


  >>

  \layout {}
}
