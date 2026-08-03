songtempo = {{{song.info.tempo}}}

% Guitar articulation marks used across the song corpus. \mypull and \myrelease
% take the grace note and the note it slurs into; \mypulled marks a single note.
mypull =
#(define-scheme-function
  (na nb)
  (ly:music? ly:music?)
  #{
    \grace {$na ^\markup {\char ##x27B6 }} $nb
  #})

mypulled =
#(define-scheme-function
  (na)
  (ly:music?)
  #{
    $na ^\markup {\char ##x27B6 }
  #})

myrelease =
#(define-scheme-function
  (na nb)
  (ly:music? ly:music?)
  #{
    \grace {$na ^\markup {\char ##x27B4 }} $nb
  #})
