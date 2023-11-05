\version "2.20.0"
\include "macros.ly"
song_tempo = 100

lead = {
  \absolute  {
    \override Score.SpacingSpanner.shortest-duration-space = #4.0
    d16\5 e8\5 d16\5
    e8\5 d16\5 e16\5
    e16\5 d16\5 e8\5
    d16\5 e16\5 r8
    }

}

drumbar =  \drummode {  bd4 sn4  bd4 sn4 }

drumbars = {
\repeat unfold 8 { \drumbar | }
}


drumbarhh =  \drummode {
  sn8
  sn8
  sn8
  sn8
  sn8
  sn8
  sn8
  sn8
}

drumbarshh = {
  \repeat unfold 8 {  \drumbarhh }

}



\score {
    <<
    \new TabStaff {
        \tempo 4 = \song_tempo
        \tabFullNotation
        \override Score.BarNumber.break-visibility = ##(#t #t #t)
       \repeat percent 8 {\lead}
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
                    \new DrumVoice {  \drumbarshh }
                    \new DrumVoice {  \drumbars }
                >>

            \new Staff {
                  \repeat unfold 8 {\lead}
                  \set Staff.midiMinimumVolume = #0.9
                  \set Staff.midiMaximumVolume = #0.9
                  \set Staff.midiInstrument = "electric guitar (clean)"
            }
        >>
        }
  %     \midi {
   %         \tempo 4 = \song_tempo
   % }
}

