#!/usr/bin/env sh

set -e
#set -x

pdf=chords_all.pdf
mkdir -p $(dirname $pdf)

../../bin/metapost_of_music_sheet.exe all_chords.yml

test -f $pdf  || ( echo "$pdf not found" && false )
echo FOUND : $pdf

count=$(ls $(dirname $pdf) | wc --lines )
echo $count

test $count -eq 1  || ( echo "more than one file : $count" && false )


echo DONE
