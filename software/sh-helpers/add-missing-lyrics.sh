#!/bin/sh

set -e

inputdir=$1


work() {
  songfile=$1
  ITEM=$2
cat $songfile | jq -r ".structure[] | select (.item.$ITEM != null ) | .id "  \
| while read -r id; do
  lyricsfile=$(dirname $songfile)/lyrics/$id.tex
  mkdir -p $(dirname $lyricsfile)
  [[ -f $lyricsfile ]] || ( echo "missing file ; $lyricsfile" ; echo "\\color{red}{add lyrics here}" >>  $lyricsfile )
done
}

find $inputdir -type f -name song.json | while read songfile ; do
  work $songfile Chords
  work $songfile Ref
done
