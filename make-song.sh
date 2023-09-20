#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))
exe=${here}/metapost_of_music_sheet/_build/default/bin/metapost_of_music_sheet.exe

echo "" > $here/test.txt

mkdir -p $here/tmp
rm -rf $here/tmp/*
(
  cd $here/tmp ;
  $exe $1| tee $here/test.txt
)

echo DONE