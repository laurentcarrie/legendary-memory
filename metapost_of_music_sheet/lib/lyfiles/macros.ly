

comma = #(define-event-function (parser location xy)
          (pair?)
          #{
            \tweak extra-offset #xy ^ \markup \musicglyph "comma"
          #})


mypull =
#(define-scheme-function
   (na nb)
   ( ly:music? ly:music?)
      #{
      %\grace ${na} {^\markup} { \char ##x2197 " 1 "  } $nb
      % \grace a8^\markup{\char ##x2197 "1"} $nb
      %\grace {$na \comma #'(1 . -2)} $nb
      \grace {$na ^\markup {\char ##x27B6 }} $nb
      %$nb
#})


mypulled =
#(define-scheme-function
   (na)
   ( ly:music? )
      #{
      $na ^\markup {\char ##x27B6 }
#})


myrelease =
#(define-scheme-function
   (na nb)
   ( ly:music? ly:music?)
      #{
      %\grace ${na} {^\markup} { \char ##x2197 " 1 "  } $nb
      % \grace a8^\markup{\char ##x2197 "1"} $nb
      %\grace {$na \comma #'(1 . -2)} $nb
      \grace {$na ^\markup {\char ##x27B4 }} $nb
      %$nb
#})
