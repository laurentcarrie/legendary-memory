let make_make_lytex : string =
  {whatever|#!/bin/bash

#set -e
#set -x

RED="\e[31m\e[47m"
GREEN='\033[0;32m\e[46m'
CYAN='\033[0;36m'
GREY="\e[37m"
NC='\033[0m' # No Color
workdir=$(dirname $(realpath $1))

rm $1.lytex
#echo "\version \"2.24.2\"" > $1.lytex
echo "\lilypondfile{$1.ly}" >> $1.lytex

fstdout=$1.lytex.stdout
fstderr=$1.lytex.stderr

printf "${GREY}building lilypond in${NC} ${CYAN}$workdir/$1$NC\n"
lilypond-book --pdf --latex-program=lualatex $1.lytex  1> $fstdout 2> $fstderr || true
lilypond-book --pdf --latex-program=lualatex $1.lytex  1>> $fstdout 2>> $fstderr
if test "x$?" = "0" ; then
  #rm $1.lytex
  printf "${GREY}building lilypond in${NC} ${GREEN}$workdir/$1$NC done\n"
else
  count=$(grep -i error $fstderr | wc --lines)
  if test "x$count" != "x0" ; then
    printf "${GREY}building lilypond in${NC} ${RED}$workdir/$1$NC error\n"
    cat $fstderr
  else
    printf "${GREY}building lilypond in${NC} ${GREEN}$workdir/$1$NC done\n"
  fi
fi
  |whatever}

let make_make_mpost : string =
  {whatever|#!/bin/bash

set -e
#set -x

mpost --tex=latex $1 1> $1.mpost.stdout.log 2> $1.mpost.stderr.log
  |whatever}

let make_make_pdf : string =
  {whatever|#!/bin/sh

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

printf "building pdf in ${GREEN}$workdir$NC done.\n"
  |whatever}

let make_make_wav : string =
  {whatever|#!/bin/bash

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

printf "building ${GREEN}$workdir/$1.wav$NC done.\n"
  |whatever}

let make_make_clean : string =
  {whatever|#!/usr/bin/env sh

#set -e
#set -x

rm -rf *.pdf *.log *.aux mps *.mpx *.dep *.lytex *.wav *.stdout *.stderr *.midi lock
find . -name "*.eps" | while read f ; do echo $f ; rm -rf $(dirname $f) ; done
  |whatever}
