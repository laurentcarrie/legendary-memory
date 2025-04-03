#!/bin/bash

#set -e
#set -x

here=$(dirname $(realpath $0))

source $here/colors.sh

p=$(which lilypond-book)
if test "x$p" = "x" ;then
  echo "!!! ERROR : lilypond is not in the path"
  exit 1
fi

lyfile=$1
lyfilenoext=$(echo $lyfile | sed "s/\.ly$//")
lytexfile=${lyfilenoext}.lytex

if ! test -f $lyfile ; then
  pwd
  printfc FAILED "file not found" $lyfile
  exit 1
fi



rm -f $lytexfile
#echo "\version \"2.24.2\"" > $lytexfile
echo "\lilypondfile{$lyfile}" >> $lytexfile

fstdout=$1.lytex.stdout
fstderr=$1.lytex.stderr

rm -rf $1.output
lilypond-book --output ${lyfilenoext}.output --pdf --latex-program=lualatex $lytexfile  1> $fstdout 2> $fstderr || true
lilypond-book --output ${lyfilenoext}.output --pdf --latex-program=lualatex $lytexfile  1>> $fstdout 2>> $fstderr || true
if test "x$?" = "0" ; then
  exit 0
else
  count=$(grep -i failed $fstderr | wc -l | sed 's/ //g')
  if test "x$count" != "x0" ; then
    exit 1
  else
    exit 0
  fi
fi
