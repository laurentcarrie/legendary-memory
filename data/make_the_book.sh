#!/bin/sh

set -e
# set -x


# Reset
Color_Off='\033[0m'       # Text Reset
Red='\033[0;31m'          # Red
Green='\033[0;32m'        # Green



here=$(dirname $(realpath $0))
songdir=$here/songs

ymlfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)

echo "" > $ymlfile
yq -i '.title="Mon Song Book"' $ymlfile
yq -i '.songs= []' $ymlfile
yq -i '.lyrics_only=false' $ymlfile
yq -i '.cover_image= true' $ymlfile

find $songdir -name song.yml | while read f ; do
    author=$(yq ".info.author" $f)
    title=$(yq ".info.title" $f)
    # yq -i "songs += [(.author=\"$author\",.title=\"$title\")]" $ymlfile
    yq -i ".songs += [{\"author\":\"$author\",\"title\":\"$title\"}] " $ymlfile
    # break
done

yq -i ".songs |= sort_by(.author,.title)" $ymlfile

old_ymlfile=$here/books/mon-song-book.yml

if ! diff $old_ymlfile $ymlfile >/dev/null; then
    printf "${Red}$old_ymlfile updated${Color_Off}\n"
    mv $ymlfile $old_ymlfile
    exit 1
else
    printf "${Green}$old_ymlfile not changed${Color_Off}\n"    
fi
exit 0
