#!/bin/bash

tmpresultfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)

echo 0 > $tmpresultfile

work() {

	here=$(dirname $realpath $1)
#	echo $here
	jsonfile=$here/song.json
	tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)


	for f in $(jq -r ".lilypondfiles[]" $jsonfile) ; do
		#  echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	for f in $(jq -r ".texfiles[]" $jsonfile) ; do
		#  echo "--------> $1 ; $f"
		md5sum $here/$f >> $tmpfile
	done

	new_digest=$(md5sum $tmpfile | sed "s/ .*//")
	old_digest=$(jq -r ".digest " $jsonfile)

	today=$(date +"%Y-%m-%d")

	if test "x$new_digest" != "x$old_digest" ; then
		echo "changed $here"
		#echo "new digest : $new_digest"
		#echo "old digest : $old_digest"
		j=$(jq . $jsonfile | jq ".digest=\"$new_digest\"" | jq ".date=\"$today\"")
		echo $j | jq "." > $jsonfile
		echo 1 > $tmpresultfile
	fi


	jq "." $jsonfile > $tmpfile
	diff $jsonfile $tmpfile
	if test "x$(cat $jsonfile)" != "x$(cat $tmpfile)" ; then
		cp $tmpfile $jsonfile
		echo 1 > $tmpresultfile
	fi
}

find songs -name "song.json" | while read f ; do
  work $f
done

ret=$(cat $tmpresultfile)
if test "x$ret" = "x0"; then
  exit 0
fi

exit 1
