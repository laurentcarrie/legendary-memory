\version "2.23.1"
\include "../../../macros.ly"
song_tempo = 100


song_chords = \chordmode {
  c2:m f2:m7 | bes2 ees2 |
}


lead = {
  \absolute  {
    \override Score.SpacingSpanner.shortest-duration-space = #4.0

    \repeat volta 3 {
      \bar ".|:"
      \set Score.currentBarNumber = 1
      |
      % mesure 1
      < c'\4 ees'\3 g'\2 c''\1>8
      < c'\4 ees'\3 g'\2 c''\1>8

      < \deadNote c'\4 \deadNote ees'\3 \deadNote g'\2 \deadNote c''\1>16
      < c'\4 ees'\3 g'\2 c''\1>16
      r8

      < c'\4 ees'\3 g'\2 c''\1>8
      r8

      < c'\4 ees'\3 aes'\2 c''\1>16
      q16
      %< c'\4 ees'\3 aes'\2 c''\1>16
      < \deadNote c'\4 \deadNote ees'\3 \deadNote aes'\2 \deadNote c''\1>16
      < \deadNote c'\4 \deadNote ees'\3 \deadNote aes'\2 \deadNote c''\1>16


      |
      % mesure 2
      < bes\4 d'\3 f'\2 bes'\1>8
      < bes\4 d'\3 f'\2 bes'\1>8

      < \deadNote bes\4 \deadNote d'\3 \deadNote f'\2 \deadNote bes'\1>8
      < bes\4 d'\3 f'\2 bes'\1>8

      < \deadNote ees\5 \deadNote bes\4 \deadNote ees'\3 \deadNote g'\2 >16
      < \deadNote ees\5 \deadNote bes\4 \deadNote ees'\3 \deadNote g'\2 >16
      < ees\5 bes\4 ees'\3 g'\2 >8

      < ees\5 bes\4 ees'\3 g'\2 >16
      < ees\5 bes\4 ees'\3 g'\2 >16
      < \deadNote ees\5 \deadNote bes\4 \deadNote ees'\3 \deadNote g'\2 >16
      < \deadNote ees\5 \deadNote bes\4 \deadNote ees'\3 \deadNote g'\2 >16

      ^\markup { \bold "3×" }
    }
  }
}

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
    \new ChordNames {
      \song_chords
    }

    \new TabStaff {
      \tempo 4 = \song_tempo
      \tabFullNotation
      \override Score.BarNumber.break-visibility = ##(#t #t #t)
      \lead
    }

  >>

  \layout {}
}
