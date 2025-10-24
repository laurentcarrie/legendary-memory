#!/bin/bash

# set -x

here=$(realpath $(dirname $0))
source $here/colors.sh

songdir=$1
printf "songdir ;  ${Red}$songdir${Color_Off}\n"

[[ -n $songdir ]]
[[ -d $songdir ]]

tmpresultfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)

echo 0 > $tmpresultfile

xxxwork() {

	here=$(dirname $(realpath $1))
	ymlfile=$here/song.yml
	tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)


	for f in $(yq ".lilypondfiles[]" $ymlfile) ; do
		# echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	for f in $(yq ".texfiles[]" $ymfile) ; do
		# echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	yq '.structure[] | select ( .item | tag == "!Chords" ) | .id ' $ymlfile | while read -r id; do
    	lyricsfile=$(dirname $ymlfile)/lyrics/$id.tex
    	md5sum $lyricsfile >> $tmpfile
  	done

	yq  '.structure[] | select ( .item | tag == "!Ref" ) | .id ' $ymlfile | while read -r id; do
    	lyricsfile=$(dirname $ymlfile)/lyrics/$id.tex
    	md5sum $lyricsfile >> $tmpfile
  	done

	# echo "x45"

	md5sum $here/add.tikz >> $tmpfile


	new_digest=$(md5sum $tmpfile | sed "s/ .*//")
	old_digest=$(yq ".digest " $ymlfile)
	old_date=$(yq ".date " $ymlfile)
	# echo "old date : '$old_date'"

	if test "x$old_date" == "xnull" ; then
		old_digest=null
	fi
	# echo "old date : '$old_date'"

	today=$(date +"%Y-%m-%d")
	# cp $ymlfile $tmpfile

	# echo "$new_digest"
	# echo "$old_digest"

	if test "x$new_digest" != "x$old_digest" ; then
		# echo "changed $here"
		#echo "new digest : $new_digest"
		#echo "old digest : $old_digest"
		author=$(yq ".author" $ymlfile)
		title=$(yq ".title" $ymlfile)
		printf "date updated : ${Red}$author${Color_Off} $Blue$title$Color_Off in $Yellow$here$Color_Off\n"
		yq -i ".digest=\"$new_digest\"" $ymlfile
		yq -i ".date=\"$today\"" $ymlfile
		echo 1 > $tmpresultfile
	else
		echo "not changed $here"
	fi

	# echo "x"

	# if  test "x$(cat $tmpfile)" != "x$(cat $ymlfile)" ; then
	# 	printf "changed: ${Red}$author${Color_Off} $Blue$title$Color_Off in $Yellow$here$Color_Off\n"
	# 	cp $tmpfile $ymlfile
	# 	echo 1 > $tmpresultfile
	# fi
	# echo "end work"
}

blah() {
	echo $1
}

# find $songdir -name "song.yml" | while read f ; do
# 	echo $f
# done

find "$songdir" -name "song.yml" -type f | while read fff ; do
#   echo "uuuuu : $fff"
#   echo "found song : $fff"
  xxxwork $fff
#   blah $fff
#   echo "z"
done

# echo "x92"

ret=$(cat $tmpresultfile)
echo "$ret"
if test "x$ret" = "x0"; then
  exit 0
fi
# echo "x98"
exit 1
