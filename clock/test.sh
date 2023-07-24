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
}

make(){

  seq 359 | while read counter
  do
      echo $counter
      cat xx.tex | sed "s/@I@/$counter/g" > yy$counter.tex
  done
  mpost --mem=metafun --tex=lualatex clock.mp
#  lualatex b.tex
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
