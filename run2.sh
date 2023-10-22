#!/bin/bash

set -e
set -x

here=$(dirname $(realpath $0))
exe=$here/metapost_of_music_sheet/_build/default/bin/metapost_of_music_sheet.exe
test -f $exe
builddir=$here/build-songs
songs=$here/songs
test -d $songs


dune build @fmt --auto-promote --root $here/metapost_of_music_sheet
dune build --root $here/metapost_of_music_sheet

$exe $builddir $songs
