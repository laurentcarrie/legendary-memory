#!/bin/bash

set -e
set -x

here=$(dirname $(realpath $0))
exe=$here/song_book_builder/_build/default/bin/song_book_builder.exe
builddir=$here/build-songs
songs=$here/songs
test -d $songs

build() {
  rm -rf $builddir
  mkdir $builddir
  pre-commit run --all-files
  dune build @fmt --auto-promote --root $here/song_book_builder
  dune build --root $here/song_book_builder
  test -f $exe
}

make() {
  $exe $builddir $songs
  (cd $builddir && omake delivery -j 4 -k)
}

case $1 in
b)
  build
  make
  ;;
m)
  make
  ;;
l)
  $exe $builddir $songs
  find $builddir -name "*$2*" -type d | while read f ; do echo $f ; \
   (cd $f && omake pdf ) ; done  ;;
s)
  $exe $builddir $songs
  (cd $builddir/$2 && omake delivery -j 4 -k )
  ;;
c)
  $exe $builddir $songs
  find $builddir -name "*$2*" -type d | while read f ; do echo $f ; \
   (cd $f && omake clean ) ; done
  ;;

*)
  echo "bad choice"
  exit 1
  ;;
esac
