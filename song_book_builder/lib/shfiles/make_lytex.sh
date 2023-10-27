#!/bin/bash

#set -e
#set -x

RED="\e[31m\e[47m"
GREEN='\033[0;32m\e[46m'
CYAN='\033[0;36m'
GREY="\e[37m"
NC='\033[0m' # No Color
workdir=$(dirname $(realpath $1))

echo "\lilypondfile{$1.ly}" > $1.lytex

fstdout=$1.lytex.stdout
fstderr=$1.lytex.stderr

printf "${GREY}building lilypond in${NC} ${CYAN}$workdir/$1$NC\n"
lilypond-book --latex-program=lualatex $1.lytex  1> $fstdout 2> $fstderr || true
lilypond-book --latex-program=lualatex $1.lytex  1>> $fstdout 2>> $fstderr
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
