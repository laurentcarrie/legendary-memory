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

\score {
    <<
    \new TabStaff {
        \tempo 4 = \song_tempo
        \tabFullNotation
        \override Score.BarNumber.break-visibility = ##(#t #t #t)
        \repeat percent 8 {\lead}
    }

    >>

    \layout {}
}
