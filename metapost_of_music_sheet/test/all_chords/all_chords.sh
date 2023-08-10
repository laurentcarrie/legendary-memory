#!/usr/bin/env sh

set -e
#set -x

pdf=main.pdf
mkdir -p $(dirname $pdf)

../../bin/metapost_of_music_sheet.exe all_chords.yml

test -f $pdf  || ( echo "$pdf not found" && false )
echo FOUND : $pdf

echo DONE
