#!/bin/bash

set -e
#set -x

here=$(dirname $(realpath $0))
source $here/colors.sh
lyfile=$1.ly

RED="\e[31m\e[47m"
GREEN='\033[0;32m'
CYAN='\033[0;36m'
GREY="\e[37m"
NC='\033[0m' # No Color

workdir=$(dirname $(realpath $lyfile))
pprintworkdir=$(echo $workdir | sed "s#$here##")
pprintworkdir="${pprintworkdir}#${lyfile}"

printfc RUN lilypond $pprintworkdir

# printf "building ${CYAN}$workdir/$1.wav$NC ...\n"

printf "${CYAN}lilypond $1$NC ...\n"
rm -f $1.midi
lilypond $1 1>$1.wav.stdout 2>$1.wav.stderr || true
printfc OK lilypond $pprintworkdir
#
#
#test -f $1.midi
printfc RUN fluidsynth $pprintworkdir
# printf "${CYAN}fluidsynth $1.midi$NC ...\n"
rm -f $1.wav
fluidsynth --gain 4 -F $1.wav /usr/share/sounds/sf2/FluidR3_GM.sf2  $1.midi 1>>$1.wav.stdout 2>>$1.wav.stderr || true
test -f $1.wav
printfc OK fluidsynth $pprintworkdir


# printf "building ${GREEN}$workdir/$1.wav$NC done.\n"
