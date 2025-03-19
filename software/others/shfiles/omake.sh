#!/bin/sh

set -e
#set -x


export build_id=$1
here=$(dirname $(realpath $0))
source $here/colors.sh
printf "songdir is ${Red}$songdir${Color_Off}\n"
printf "bookdir is ${Red}$bookdir${Color_Off}\n"

printf "${Green}check json${Color_Off}\n"
bash $here/check-json.sh $bookdir 1>>omake.$build_id.stdout 2>>omake.$build_id.stderr
printf "${Green}add missing lyrics${Color_Off}\n"
bash $here/add-missing-lyrics.sh $songdir 1>>omake.$build_id.stdout 2>>omake.$build_id.stderr
printf "${Green}check existence of files${Color_Off}\n"
bash $here/check-existence-of-files.sh $songdir 1>>omake.$build_id.stdout 2>>omake.$build_id.stderr

printf "${Green}run omake in $here ${Color_Off}\n"
( cd $here && omake -j 8 -k 1>>omake.$build_id.stdout 2>>omake.$build_id.stderr )

printf "$Color_Off"
