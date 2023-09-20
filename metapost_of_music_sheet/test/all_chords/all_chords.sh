#!/usr/bin/env sh

set -e
#set -x

here=$(dirname $(realpath $0))
pdf=$here/all_chords.pdf
#mkdir -p $(dirname $pdf)


../../bin/metapost_of_music_sheet.exe $here/all_chords.yml

test -f $pdf  || ( echo "$pdf not found" && false )
echo FOUND : $pdf

echo DONE
