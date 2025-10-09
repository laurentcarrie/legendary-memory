#!/bin/sh

set -e

# set -x

here=$(dirname $(realpath $0))
songdir=$here/songs


doc="[]"

ftmp=$(mktemp)
# ftmp=a.json
echo $doc > $ftmp

find $songdir -name song.json | while read f ; do
    data=$(cat $f | jq ". | {author,title}")
    # doc=$(cat $ftmp | jq ".songs += [$data]" )
    doc=$(cat $ftmp | jq ". += [$data]" )
    echo $doc > $ftmp
done
sorted_songs=$(cat $ftmp | jq -r ". | sort_by(.author,.title)")
doc="{\"title\": \"Mon Song Book\",\"songs\":$sorted_songs,\"cover_image\": true,\"lyrics_only\": false}"
echo $doc | jq "." > $ftmp

bookfile=$here/books/my-song-book.json

if ! diff $bookfile $ftmp >/dev/null; then
    echo "$bookfile updated"
    mv $ftmp $bookfile
    exit 1
else
    echo "$bookfile not changed"    
fi
exit 0
