\version "2.20.0"

song_tempo = 114


\paper {
  #(include-special-characters)
  indent = 0\mm
  line-width = 80\mm
  oddHeaderMarkup = ""
  evenHeaderMarkup = ""
  oddFooterMarkup = ""
  evenFooterMarkup = ""
  ragged-right = ##f

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
      f1 | f1 | a1:m7 | a1:m7  \break
      bes1:7 | bes1:7 | |
    }



  >>

  \layout {}
}
