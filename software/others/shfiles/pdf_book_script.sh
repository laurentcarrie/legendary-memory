#!/usr/bin/env bash

set -e
#set -x

if test x"$SONGBOOOK_DEBUG" != "x" ; then
  set -x
fi

builddir=$1
here=$(dirname $(realpath $0))


source $here/colors.sh

cd $builddir



title=$(cat $builddir/book.json | jq -r ".title")
path=$(cat $builddir/book.json | jq -r ".path")
pdfname=$(cat $builddir/book.json | jq -r ".pdfname")
pdffile=$builddir/${pdfname}.pdf

rm -f main.aux main.log main.out main.toc


make_digest() {
   if test -f $pdffile ; then
    tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)
    cat $builddir/book.json | jq -c ".songs[]" | while read dep ; do
      songpath=$(echo $dep | jq  -r ".path" )
      songpdfname=$(echo $dep | jq  -r ".pdfname" )
      songpdffile=${songpath}/${songpdfname}.pdf
      test -f $songpdffile
      md5sum $songpdffile >> $tmpfile
    done
    cat $tmpfile > debug0.txt
    md5sum $pdffile >> $tmpfile
    cat $tmpfile > debug1.txt
    md5sum $tmpfile | sed "s/ .*//"
  else
    echo "no $pdffile"
  fi
}


new_digest=$(make_digest)
checksumfile_ok=$builddir/.checksum_ok
old_digest_ok=$(cat $checksumfile_ok 2>/dev/null || echo "missing")
checksumfile_failed=$builddir/.checksum_failed
old_digest_failed=$(cat $checksumfile_failed 2>/dev/null || echo "missing")

if test "x$new_digest" = "x$old_digest_ok" ; then
  printf "[${White}${On_Green}NoNeed${Color_Off}][$Green$title$Color_Off] book\n"
  exit 0
fi

if test "x$new_digest" = "x$old_digest_failed" ; then
  printf "[${On_Red}NoNeed${Color_Off}][$Green$title$Color_Off] book   \n"
  exit 0
fi



printf "[${Blue} Start${Color_Off}][${Green}$title${Color_Off}] book \n"


# printf "builddir is $builddir\n"
test -f "main.tex" || (printf "could not find main.tex\n" && exit 1)
i=1
while [ $i -lt 10 ]
do
  printf "[${Blue} ($i)  ${Color_Off}][$Green$title$Color_Off] $title\n"
  lualatex main.tex 1>pdf.stdout 2> pdf.stderr
  # lualatex main.tex
  test -f main.log
  count=$(cat main.log | grep "Rerun to get" | wc -l)
  if test "x$count" = "x0" ; then
    break
  fi
  i=$[$i+1]
done


if test -f main.pdf ; then
  mv main.pdf $pdffile
  new_digest=$(make_digest)
  echo $new_digest > $checksumfile_ok
  rm -f $checksumfile_failed
  printf "[${Black}${On_Green}  OK  ${Color_Off}][$Green$title$Color_Off] build pdf Ok ($i passes)\n"
  exit 0
else
  echo $new_digest > $checksumfile_failed
  rm -f $checksumfile_ok
  printf "[${Black}${On_Red}FAILED${Color_Off}][$Green$title]$Color_Off build pdf failed\n"
  exit 1
fi
