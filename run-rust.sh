#!/bin/bash

set -e
set -x

here=$(dirname $(realpath $0))
builddir=$here/build-rust
songs=$here/songs
test -d $songs

build() {
  (
  cd $here/rust/songs
  bash ./rs_of_others.sh
  cargo fmt
  cargo build
  )
}

make() {
  ( cd $here/rust/songs && cargo run -- $songs $builddir )
  (cd $builddir && omake delivery -j 4 -k)
}

format() {
  cd $here && bash my-latexindent.sh
}

gdrive() {
  (cd $builddir && omake gdrive)
}

dev(){
  rm -rf $builddir
  build
  make
  format
}


all(){
  rm -rf $builddir
  build
  make
  gdrive
}

case $1 in
b)
  build
  make
  format
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
gd)
  gdrive
  ;;
dev)
  dev
  ;;
all)
  all
  ;;

*)
  echo "bad choice [b|m|c|s|gd|dev|all]"
  exit 1
  ;;
esac
