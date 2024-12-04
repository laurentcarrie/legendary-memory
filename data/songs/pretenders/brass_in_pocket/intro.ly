\version "2.23.1"
\include "../../../macros.ly"
song_tempo = 100



lead = {
  \absolute  {
    \override Score.SpacingSpanner.shortest-duration-space = #4.0
    \set Score.currentBarNumber = 1

    <a\4 cis'~\3 b\2>16
    <d'\3>16
    <a\4 d'\3 b\2 >16
    <a\4 d'~\3 b~\2 e'~\1>16

    <d'\3 b\2 e'\1>16
    <d'\3 b\2 e'\1>16
    <a~\4 cis'~\3 b~\2 e'~\1>8

    <a\4 cis'\3 b\2 e'\1>2


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

  %  #(add-text-replacements!
  %    '(
  %       ("100" . "hundred")
  %       ("dpi" . "dots per inch")
  %      ))

}


\score {
  <<
    \new TabStaff {
      \tempo 4 = \song_tempo
      \tabFullNotation
      \override Score.BarNumber.break-visibility = ##(#t #t #t)
      \lead
    }

  >>

  \layout {
    #(layout-set-staff-size 20)
  }
}
