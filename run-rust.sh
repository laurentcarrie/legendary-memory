#!/bin/bash

set -e
set -x

here=$(dirname $(realpath $0))
builddir=$here/build-rust
songs=$here/songs
books=$here/books

printf "songs: $songs\n"
[[ -n $songs ]]
[[ -d $songs ]]

printf "books: $books\n"
[[ -n $books ]]
[[ -d $books ]]

build() {
  (
  cd $here/rust/songs
  bash ./rs_of_others.sh
  cargo fmt
  cargo build
  )
}

make() {
  ( cd $here/rust/songs && cargo run -- $songs $books $builddir )
  (cd $builddir && omake delivery -j 4 -k)
}

format() {
  cd $here && bash my-latexindent.sh
}

gdrive() {
  (cd $builddir && omake gdrive )
}

dev(){
  rm -rf $builddir
  build
  make
  format
}


#book() {
#  cd $builddir
#  convert \
#  amy_winehouse--@--you_know_i_m_no_good.pdf \
#  maroon_5--@--this_love.pdf 13-fevrier.pdf  \
#  estelle___k__west___cocoon--@--american_boy.pdf \
#  natalie_imbroglia--@--torn.pdf \
#  red_hot_chili_peppers--@--can_t_stop.pdf \
#  alannah_myles--@--black_velvet.pdf \
#  concert-13-fevrier.pdf
#}

all(){
  rm -rf $builddir
  build
  make
#  gdrive
}

deploy(){
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
deploy)
  deploy
  ;;

*)
  echo "bad choice [b|m|c|s|gd|dev|all]"
  exit 1
  ;;
esac
