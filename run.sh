#!/bin/bash

set -e
set -x

here=$(dirname $(realpath $0))




pre-commit run --all-files

exe=$here/song_book_builder/_build/default/bin/song_book_builder.exe
builddir=$here/build-songs
songs=$here/songs
test -d $songs


dune build @fmt --auto-promote --root $here/song_book_builder
dune build --root $here/song_book_builder
test -f $exe

$exe $builddir $songs
