#!/bin/bash

set -e
#set -x
here=$(realpath $(dirname $0))

work() {
	here=$(dirname $(realpath $1))
	jsonfile=$here/song-internal.json
	tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)


	for f in $(jq -r ".lilypondfiles[]" $jsonfile) ; do
		#  echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	for f in $(jq -r ".texfiles[]" $jsonfile) ; do
		#  echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	cat $jsonfile | jq -r ".structure[] | select (.item.Chords != null ) | .id "  | while read -r id; do
    lyricsfile=$(dirname $jsonfile)/lyrics/$id.tex
    md5sum $lyricsfile >> $tmpfile
  done

  cat $jsonfile | jq -r ".structure[] | select (.item.Ref != null ) | .id "  | while read -r id; do
    lyricsfile=$(dirname $jsonfile)/lyrics/$id.tex
    md5sum $lyricsfile >> $tmpfile
  done


	digest=$(md5sum $tmpfile | sed "s/ .*//")

  echo $digest

}

work $1
