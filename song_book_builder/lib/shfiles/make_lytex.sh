#!/bin/bash

#set -e
set -x

RED="\e[31m\e[47m"
GREEN='\033[0;32m\e[46m'
CYAN='\033[0;36m'
GREY="\e[37m"
NC='\033[0m' # No Color

lyfile=$1.ly
lytexfile=$1.lytex

if ! test -f $lyfile ; then
  printf "not found : ${RED}$lyfile$NC\n"
  exit 1
fi


workdir=$(dirname $(realpath $lyfile))

rm -f $lytexfile
#echo "\version \"2.24.2\"" > $lytexfile
echo "\lilypondfile{$lyfile}" >> $lytexfile

fstdout=$1.lytex.stdout
fstderr=$1.lytex.stderr

printf "${GREY}building lilypond in${NC} ${CYAN}$workdir/$1$NC\n"
rm -rf $1.output
lilypond-book --output $1.output --pdf --latex-program=lualatex $lytexfile  1> $fstdout 2> $fstderr || true
lilypond-book --output $1.output --pdf --latex-program=lualatex $lytexfile  1>> $fstdout 2>> $fstderr || true
if test "x$?" = "0" ; then
  printf "${GREY}building lilypond in${NC} ${GREEN}$workdir/$1$NC done\n"
else
  count=$(grep -i failed $fstderr | wc -l | sed 's/ //g')
  if test "x$count" != "x0" ; then
#    rm $lytexfile
#    rm $1.tex
#    rm -f $1.dep
#    rm -rf $1.output
    printf "${GREY}building lilypond in${NC} ${RED}$workdir/$1$NC failed\n"
    #cat $fstdout
#    cat $fstderr
    #exit 1
  else
    printf "${GREY}building lilypond in${NC} ${GREEN}$workdir/$1$NC done\n"
  fi
fi
