#!/bin/sh

set -e

# set -x

here=$(dirname $(realpath $0))
songdir=$here/songs


doc="[]"

ftmp=$(mktemp)
echo $doc > $ftmp

find $songdir -name song.json | while read f ; do
    data=$(cat $f | jq ". | {author,title}")
    # doc=$(cat $ftmp | jq ".songs += [$data]" )
    doc=$(cat $ftmp | jq ". += [$data]" )
    echo $doc > $ftmp
done
sorted_songs=$(cat $ftmp | jq -r ". | sort_by(.author,.title)")
doc="{\"title\": \"The Book\",\"songs\":$sorted_songs}"

bookfile=$here/books/the-book.json

if diff $bookfile $ftmp >/dev/null ; then
    echo "$bookfile updated"
    echo $olddata > olddata.json
    echo $newdata > newdata.json
    echo $newdata > $bookfile
    exit 1
else
    echo "$bookfile not changed"    
fi
exit 0
