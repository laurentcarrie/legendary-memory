#!/bin/bash

set -e
set -x

clean() {
  rm -f *.pdf
  rm -f *.mps
  rm -f c-*
  rm -f a-*
  rm -f *.log
  rm -f *.mpo
  rm -f *.aux
  rm -f mpxerr.tex
  rm -f mptextmp.mp
  rm -rf *.png
  rm -rf yy*.tex

  rm -rf mps
  mkdir mps

  rm -rf pdftex
  mkdir pdftex
}

make(){
  mpost --mem=metafun --tex=lualatex clock.mp
  rm -f timeline.txt
  touch timeline.txt


  MAX=10

  seq $MAX | while read counter
  do
      echo $counter
      cat clock.tex | sed "s/@I@/$counter/g" > pdftex/clock-$counter.tex
      (cd pdftex && lualatex clock-$counter.tex)
      echo "::$counter" >> pdftex/timeline.txt
  done
  cat anim.tex | sed "s/@MAX@/$MAX/g" > pdftex/anim.tex
  (cd pdftex && lualatex anim.tex )
  mv pdftex/anim.pdf .
  rm -rf mps pdftex
}

case $1 in
cm)
  clean
  make
  ;;
c)
  clean
  ;;
m)
  make
  ;;
*)
  echo "bad command"
  exit 1
esac
