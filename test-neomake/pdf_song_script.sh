#!/usr/bin/env bash

#set -e

builddir=$1
here=$(dirname $(realpath $0))

author=$(cat $builddir/song.json | jq -r ".author")
title=$(cat $builddir/song.json | jq -r ".title")

source $here/colors.sh


myprint() {
    printf "[${On_Green}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off \n"
}


new_digest=$(bash md5sum.sh $1/song.json)
#printf "new digest : $new_digest\n"
old_digest_ok=""
old_digest_failed=""
checksumfile_ok=$builddir/.checksum_ok
checksumfile_failed=$builddir/.checksum_failed
if test -f $checksumfile_ok ; then
  old_digest_ok=$(cat $checksumfile_ok)
fi
if test -f $checksumfile_failed ; then
  old_digest_failed=$(cat $checksumfile_failed)
fi
#printf "old digest : $old_digest\n"

if test "x$new_digest" = "x$old_digest_ok" ; then
  printf "[${On_Green}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off\n"
  exit 0
fi

if test "x$new_digest" = "x$old_digest_failed" ; then
  printf "[${On_Red}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off \n"
  exit 0
fi


cd $builddir


cat $builddir/song.json | jq -r ".lilypondfiles[]" | while read lyfile ; do
#  printf "[$Green$author$Color_Off][${Blue}$title$Color_Off] $lyfile \n"
  ( bash $here/lytex_script.sh $lyfile &&
    printf "[${On_Green}  OK  ${Color_Off}][$Green$author$Color_Off][${Blue}$title$Color_Off] $lyfile \n"
  ) || (
    printf "[${Red}${On_Yellow}FAILED${Color_Off}][$Green$author$Color_Off][${Blue}$title$Color_Off] $lyfile \n"
  )
done

printf "[${Blue}Start${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off \n"


# printf "builddir is $builddir\n"
test -f "main.tex" || (printf "could not find main.tex\n" && exit 1)
i=0
while [ $i -lt 4 ]
do
  lualatex main.tex 1>pdf.stdout 2> pdf.stderr
  # lualatex main.tex
  test -f main.log
  count=$(cat main.log | grep Rerun | wc -l)
  if test "x$count" = "x0" ; then
    break
  fi
  i=$[$i+1]
done

if test -f main.pdf ; then
  echo $new_digest > $checksumfile_ok
  rm -f $checksumfile_failed
  printf "[${On_Green}  OK  ${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off build pdf Ok\n"
else
  echo $new_digest > $checksumfile_failed
  rm -f $checksumfile_ok
  printf "[${On_Red}FAILED${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off build pdf failed\n"
  exit 0
fi
