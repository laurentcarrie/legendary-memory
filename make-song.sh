#!/bin/sh

set -e
set -x
set -o pipefail

here=$(dirname $(realpath $0))
exe=${here}/metapost_of_music_sheet/_build/default/bin/metapost_of_music_sheet.exe

echo "" > $here/test.txt

input=$(realpath $1)

mkdir -p $here/tmp
rm -rf $here/tmp/*
(
  cd $here/tmp ;
  $exe $input | tee $here/test.txt
)

echo DONE
