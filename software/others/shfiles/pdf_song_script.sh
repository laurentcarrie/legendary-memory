#!/usr/bin/env bash

set -e
#set -x

if test x"$SONGBOOOK_DEBUG" != "x" ; then
  set -x
fi

builddir=$1
here=$(dirname $(realpath $0))

author=$(cat $builddir/song-internal.json | jq -r ".author")
title=$(cat $builddir/song-internal.json | jq -r ".title")
pdfname=$(cat $builddir/song-internal.json | jq -r ".pdfname")
srcdir=$(cat $builddir/song-internal.json | jq -r ".srcdir")
pdffile=$builddir/${pdfname}.pdf

source $here/colors.sh

myprint() {
    printf "[${On_Green}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off \n"
}

# the md5sum.sh takes a digest of the sources ( json, tex, ly files )
# we add the digest of the pdffile if it exists
make_digest() {
   if test -f $pdffile ; then
    new_digest=$(bash $here/md5sum.sh $builddir/song-internal.json)
    tmpfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)
    echo $new_digest >> $tmpfile
    md5sum $pdffile >> $tmpfile
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

#echo "new : $new_digest"
#echo "old ok :  $old_digest_ok"
#echo "old failed :  $old_digest_failed"

if test "x$new_digest" = "x$old_digest_ok" ; then
  printf "[${White}${On_Green}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off\n"
  exit 0
fi

if test "x$new_digest" = "x$old_digest_failed" ; then
  printf "[${On_Red}NoNeed${Color_Off}]$Green[$author]$Color_Off$Blue[$title]$Color_Off \n"
  exit 1
fi


cd $builddir

cat $builddir/song-internal.json | jq -r ".texfiles[]" | while read f ; do
  test -f $srcdir/$f
  cp $srcdir/$f $builddir/.
done

cp $here/songs/*.tex $builddir/.
cp $srcdir/body.tex .

mkdir -p lyrics
cat $builddir/song-internal.json | jq -r ".structure[].item.ItemChords.section_id" | while read id ; do
  if test "x$id" != "xnull" ; then
    test -f $srcdir/lyrics/$id.tex
    cp $srcdir/lyrics/$id.tex $builddir/lyrics/.
  fi
done

cat $builddir/song-internal.json | jq -r ".structure[].item.ItemRef.section_id" | while read id ; do
  if test "x$id" != "xnull" ; then
    test -f $srcdir/lyrics/$id.tex
    cp $srcdir/lyrics/$id.tex $builddir/lyrics/.
  fi
done



cat $builddir/song-internal.json | jq -r ".lilypondfiles[]" | while read lyfile ; do
#  printf "[$Green$author$Color_Off][${Blue}$title$Color_Off] $lyfile \n"
  test -f $srcdir/$lyfile
  cp $srcdir/$lyfile .
  ( bash $here/lytex_script.sh $lyfile &&
    printf "[${Green}${On_Yellow}  OK  ${Color_Off}][$Green$author$Color_Off][${Blue}$title$Color_Off] $lyfile \n"
  ) || (
    printf "[${Red}${On_Yellow}FAILED${Color_Off}][$Green$author$Color_Off][${Blue}$title$Color_Off] $lyfile \n"
  )
done

printf "[${Blue} Start${Color_Off}][$Green$author$Color_Off][$Blue$title$Color_Off] \n"


# printf "builddir is $builddir\n"
test -f "main.tex" || (printf "could not find main.tex\n" && exit 1)
i=1
while [ $i -lt 10 ]
do
  printf "[${Blue} ($i)  ${Color_Off}][$Green$author$Color_Off][$Blue$title$Color_Off] \n"
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
  # we need to recompute because it has the pdffile
  new_digest=$(make_digest)
  echo $new_digest > $checksumfile_ok
  rm -f $checksumfile_failed
  printf "[${On_Green}  OK  ${Color_Off}][$Green$author$Color_Off][$Blue$title$Color_Off] build pdf Ok ($i passes)\n"
else
  new_digest=$(make_digest)
  echo $new_digest > $checksumfile_failed
  rm -f $checksumfile_ok
  printf "[${On_Red}FAILED${Color_Off}][$Green$author$Color_Off][$Blue$title$Color_Off] build pdf failed\n"
  exit 1
fi
