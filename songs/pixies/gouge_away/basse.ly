\version "2.24.2"
\include "macros.ly"
song_tempo = 132

lead = {
  \absolute  {
    \override Score.SpacingSpanner.shortest-duration-space = #4.0
    gis,8\6  gis,8\6 gis,8\6 gis,8\6 gis,8\6 gis,8\6 gis,8\6 gis,8\6 |
    b,8\5  b,8\5 b,8\5 b,8\5 b,8\5 b,8\5 b,8\5 b,8\5 |
    e8\4  e8\4 e8\4 e8\4 e8\4 e8\4 e8\4 e8\4 |
    gis,8\6  gis,8\6 gis,8\6 gis,8\6 gis,8\6 gis,8\6 gis,8\6 gis,8\6 |
    e8\4  e8\4 e8\4 e8\4 e8\4 e8\4 e8\4 e8\4 |
    }

}

drumbar =  \drummode {  bd4 sn4  bd4 sn4 }

drumbars = {
\repeat unfold 15 { \drumbar | }
}


drumbarhh =  \drummode {
  sn8
  r8
  sn8
  r8
  sn8
  r8
  sn8
  r8
}

drumbarshh = {
  \repeat unfold 15 {  \drumbarhh }

}



\score {
    <<
    \new TabStaff {
        \tempo 4 = \song_tempo
        \tabFullNotation
        \override Score.BarNumber.break-visibility = ##(#t #t #t)
       \repeat percent 3 {\lead}
    }

    >>

   % \layout {}
}


\score {
        \unfoldRepeats {
        <<
            \new DrumStaff
                \tempo 4 = \song_tempo
                <<
                    %\new DrumVoice {  \drumbarshh }
                    \new DrumVoice {  \drumbars }
                >>

            \new Staff {
                  \repeat unfold 3 {\lead}
                  \set Staff.midiMinimumVolume = #0.9
                  \set Staff.midiMaximumVolume = #0.9
                  \set Staff.midiInstrument = "electric guitar (clean)"
            }
        >>
        }
       \midi {
            \tempo 4 = \song_tempo
    }
}
