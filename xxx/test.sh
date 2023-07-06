#!/bin/sh

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
}

make(){

  lualatex bb1.tex
  lualatex bb2.tex
  mpost --mem=metafun --tex=lualatex c.mp
  lualatex b.tex
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


