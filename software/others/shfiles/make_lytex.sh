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

lyfile=$1.ly
lytexfile=$1.lytex
#html_output=$2

if ! test -f $lyfile ; then
  printfc FAILED "file not found" $lyfile
  exit 1
fi

workdir=$(dirname $(realpath $lyfile))
pprintworkdir=$(echo $workdir | sed "s#$here##")
pprintworkdir="${pprintworkdir}#${lyfile}"


rm -f $lytexfile
#echo "\version \"2.24.2\"" > $lytexfile
echo "\lilypondfile{$lyfile}" >> $lytexfile

fstdout=$1.lytex.stdout
fstderr=$1.lytex.stderr

printfc RUN lilypond $pprintworkdir


rm -rf $1.output
lilypond-book --output $1.output --pdf --latex-program=lualatex $lytexfile  1> $fstdout 2> $fstderr || true
lilypond-book --output $1.output --pdf --latex-program=lualatex $lytexfile  1>> $fstdout 2>> $fstderr || true
if test "x$?" = "0" ; then
  printfc OK lilypond $pprintworkdir
else
  count=$(grep -i failed $fstderr | wc -l | sed 's/ //g')
  if test "x$count" != "x0" ; then
#    rm $lytexfile
#    rm $1.tex
#    rm -f $1.dep
#    rm -rf $1.output
    printfc FAILED lilypond $pprintworkdir
    #cat $fstdout
#    cat $fstderr
    #exit 1
  else
    printfc OK lilypond $pprintworkdir
  fi
fi
