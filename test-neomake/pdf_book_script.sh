#!/usr/bin/env bash

set -e
#set -x

builddir=$1
here=$(dirname $(realpath $0))


source $here/colors.sh

cd $builddir

title=$(cat book.json | jq -r ".title")

rm -f main.aux main.log main.out main.toc

tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)

cat book.json | jq -c ".songs[]" | while read dep ; do
  path=$(echo $dep | jq  -r ".path" )
  pdfname=$(echo $dep | jq  -r ".pdfname" )
  md5sum $path/${pdfname}.pdf >> $tmpfile
done

new_digest=$(md5sum $tmpfile | sed "s/ .*//")
echo $new_digest > x.tmp
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

if test "x$new_digest" = "x$old_digest_ok" ; then
  printf "[${White}${On_Green}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off\n"
  exit 0
fi

if test "x$new_digest" = "x$old_digest_failed" ; then
  printf "[${On_Red}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off \n"
  exit 0
fi



printf "[${Blue}Start${Color_Off}]${Green}[$title] \n"


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
  printf "[${On_Green}  OK  ${Color_Off}][$Green$title$Color_Off] build pdf Ok\n"
else
  echo $new_digest > $checksumfile_failed
  rm -f $checksumfile_ok
  printf "[${On_Red}FAILED${Color_Off}][$Green$title]$Color_Off build pdf failed\n"
  exit 1
fi
