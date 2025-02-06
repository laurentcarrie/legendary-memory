#!/bin/sh

set -e
# set -x

here=$(dirname $(realpath $0))
source $(dirname $here)/others/shfiles/colors.sh

inputdir=$1


work() {
  songfile=$1
  item=$2
  # printf "${Green}$songfile${Color_Off}\n"
cat $songfile |  jq -r ".$item[]" \
| while read -r f; do
  inputfile=$(echo $(dirname $songfile)/$f | sed "s/.wav/.ly/")

  [[ -f $inputfile ]] || ( printf "in ${On_Red}$songfile${Color_Off}\nsection ${On_Yellow}$item${Color_Off}\nmissing file ; ${Red}$f${Color_Off}\n" ; exit 1 )
done
}

find $inputdir -type f -name song.json | while read songfile ; do
  work $songfile texfiles
  work $songfile lilypondfiles
  work $songfile wavfiles

done
