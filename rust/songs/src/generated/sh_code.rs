pub fn make_make_lytex() -> String {
    let ret = r###"#!/bin/bash

#set -e
#set -x

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
fi"###;
    ret.to_string()
}
pub fn make_make_mpost() -> String {
    let ret = r###"#!/bin/bash

set -e
#set -x

mpost --tex=latex $1 1> $1.mpost.stdout.log 2> $1.mpost.stderr.log"###;
    ret.to_string()
}
pub fn make_make_pdf() -> String {
    let ret = r###"#!/bin/sh

set -e
#set -x
RED="\e[31m\e[47m"
GREEN='\033[0;32m'
CYAN='\033[0;36m'
GREY="\e[37m"
NC='\033[0m' # No Color
workdir=$(dirname $(realpath $1.tex))
printf "${GREY}building pdf in${NC} ${CYAN}$workdir$NC\n"

i="0"
while [ $i -lt 4 ]
do
lualatex $1.tex 1> $1.pdf.stdout.log 2> $1.pdf.stderr.log
test -f main.log
count=$(cat $1.log | grep Rerun | wc -l)
if test "x$count" = "x0" ; then
    break
fi
i=$[$i+1]
done

printf "building pdf in ${GREEN}$workdir$NC done.\n""###;
    ret.to_string()
}
pub fn make_make_wav() -> String {
    let ret = r###"#!/bin/bash

set -e
#set -x

RED="\e[31m\e[47m"
GREEN='\033[0;32m'
CYAN='\033[0;36m'
GREY="\e[37m"
NC='\033[0m' # No Color
workdir=$(dirname $(realpath $1))

printf "building ${CYAN}$workdir/$1.wav$NC ...\n"

lilypond $1 1>$1.wav.stdout 2>$1.wav.stderr
fluidsynth --gain 4 -F $1.wav /usr/share/sounds/sf2/FluidR3_GM.sf2  $1.midi 1>>$1.wav.stdout 2>>$1.wav.stderr

printf "building ${GREEN}$workdir/$1.wav$NC done.\n""###;
    ret.to_string()
}
pub fn make_make_clean() -> String {
    let ret = r###"#!/usr/bin/env sh

#set -e
#set -x

rm -rf *.pdf *.log *.aux mps *.mpx *.dep *.lytex *.wav *.stdout *.stderr *.midi lock
find . -name "*.eps" | while read f ; do echo $f ; rm -rf $(dirname $f) ; done"###;
    ret.to_string()
}
