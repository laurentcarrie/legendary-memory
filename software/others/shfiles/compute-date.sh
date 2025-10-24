#!/bin/bash

set -e
#set -x
here=$(realpath $(dirname $0))
source $here/colors.sh

songdir=$1
[[ -n $songdir ]]
[[ -d $songdir ]]

tmpresultfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)

echo 0 > $tmpresultfile

work() {

	here=$(dirname $(realpath $1))
	echo $here
	ymlfile=$here/song.yml
	tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)


	for f in $(yq ".lilypondfiles[]" $ymlfile) ; do
		#  echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	for f in $(yq ".texfiles[]" $ymfile) ; do
		#  echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	yq ".structure[].item | select ( tag == "!Chords" ) | .id " $ymlfile | while read -r id; do
    lyricsfile=$(dirname $ymlfile)/lyrics/$id.tex
    md5sum $lyricsfile >> $tmpfile
  done

	yq  ".structure[].item | select ( tag == "!Ref" ) | .id " $ymlfile | while read -r id; do
    lyricsfile=$(dirname $ymlfile)/lyrics/$id.tex
    md5sum $lyricsfile >> $tmpfile
  done

	md5sum $here/add.tikz >> $tmpfile


	new_digest=$(md5sum $tmpfile | sed "s/ .*//")
	old_digest=$(yq ".digest " $ymlfile)

	today=$(date +"%Y-%m-%d")

	if test "x$new_digest" != "x$old_digest" ; then
		echo "changed $here"
		#echo "new digest : $new_digest"
		#echo "old digest : $old_digest"
		author=$(yq ".author" $ymlfile)
		title=$(yq ".title" $ymlfile)
		printf "date updated : ${Red}$author${Color_Off} $Blue$title$Color_Off in $Yellow$here$Color_Off\n"
		yq -i '.digest="$new_digest"' $ymlfile
		yq -i '.date="$today"' $ymlfile
		echo 1 > $tmpresultfile
	fi


	jq "." $jsonfile > $tmpfile
#	diff $jsonfile $tmpfile
	if test "x$(cat $jsonfile)" != "x$(cat $tmpfile)" ; then
		cp $tmpfile $jsonfile
		echo 1 > $tmpresultfile
	fi
}

find $songdir -name "song.json" | while read f ; do
  work $f
done

ret=$(cat $tmpresultfile)
if test "x$ret" = "x0"; then
  exit 0
fi

exit 1
