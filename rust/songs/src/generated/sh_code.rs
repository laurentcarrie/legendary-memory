pub fn make_make_lytex() -> String {
    let ret = r###"#!/bin/bash

#set -e
#set -x

here=$(dirname $(realpath $0))

source $here/colors.sh

lyfile=$1.ly
lytexfile=$1.lytex

if ! test -f $lyfile ; then
  printfc FAILED "file not found" $lyfile
  exit 1
fi


workdir=$(dirname $(realpath $lyfile))
pprintworkdir=$(echo $workdir | sed "s#$here##")


rm -f $lytexfile
#echo "\version \"2.24.2\"" > $lytexfile
echo "\lilypondfile{$lyfile}" >> $lytexfile

fstdout=$1.lytex.stdout
fstderr=$1.lytex.stderr

printfc RUN lilypond $pprintworkdir


rm -rf $1.output
lilypond-book --output $1.output --pdf --latex-program=lualatex $lytexfile  1> $fstdout 2> $fstderr || true
#lilypond-book --output $1.output --pdf --latex-program=lualatex $lytexfile  1>> $fstdout 2>> $fstderr || true
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
fi"###;
    ret.to_string()
}
pub fn make_make_mpost() -> String {
    let ret = r###"#!/bin/bash

set -e
#set -x

here=$(dirname $(realpath $0))
source $here/colors.sh

workdir=$(dirname $(realpath $1.mpost))
pprintworkdir=$(echo $workdir | sed "s#$here##")

printfc RUN mpost $pprintworkdir

mpost --tex=latex $1 1> $1.mpost.stdout.log 2> $1.mpost.stderr.log

printfc OK mpost $pprintworkdir"###;
    ret.to_string()
}
pub fn make_make_pdf() -> String {
    let ret = r###"#!/bin/sh

here=$(dirname $(realpath $0))

source $here/colors.sh

#set -e
#set -x

workdir=$(dirname $(realpath $1.tex))
pprintworkdir=$(echo $workdir | sed "s#$here##")

printfc RUN pdf $pprintworkdir

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

printfc OK pdf $pprintworkdir"###;
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

printf "${CYAN}lilypond $1$NC ...\n"
rm -f $1.midi
lilypond $1 1>$1.wav.stdout 2>$1.wav.stderr || true
test -f $1.midi
printf "${CYAN}fluidsynth $1.midi$NC ...\n"
rm -f $1.wav
fluidsynth --gain 4 -F $1.wav /usr/share/sounds/sf2/FluidR3_GM.sf2  $1.midi 1>>$1.wav.stdout 2>>$1.wav.stderr || true
test -f $1.wav

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
pub fn make_colors() -> String {
    let ret = r###"# Reset
Color_Off='\033[0m'       # Text Reset

# Regular Colors
Black='\033[0;30m'        # Black
Red='\033[0;31m'          # Red
Green='\033[0;32m'        # Green
Yellow='\033[0;33m'       # Yellow
Blue='\033[0;34m'         # Blue
Purple='\033[0;35m'       # Purple
Cyan='\033[0;36m'         # Cyan
White='\033[0;37m'        # White

# Bold
BBlack='\033[1;30m'       # Black
BRed='\033[1;31m'         # Red
BGreen='\033[1;32m'       # Green
BYellow='\033[1;33m'      # Yellow
BBlue='\033[1;34m'        # Blue
BPurple='\033[1;35m'      # Purple
BCyan='\033[1;36m'        # Cyan
BWhite='\033[1;37m'       # White

# Underline
UBlack='\033[4;30m'       # Black
URed='\033[4;31m'         # Red
UGreen='\033[4;32m'       # Green
UYellow='\033[4;33m'      # Yellow
UBlue='\033[4;34m'        # Blue
UPurple='\033[4;35m'      # Purple
UCyan='\033[4;36m'        # Cyan
UWhite='\033[4;37m'       # White

# Background
On_Black='\033[40m'       # Black
On_Red='\033[41m'         # Red
On_Green='\033[42m'       # Green
On_Yellow='\033[43m'      # Yellow
On_Blue='\033[44m'        # Blue
On_Purple='\033[45m'      # Purple
On_Cyan='\033[46m'        # Cyan
On_White='\033[47m'       # White

# High Intensity
IBlack='\033[0;90m'       # Black
IRed='\033[0;91m'         # Red
IGreen='\033[0;92m'       # Green
IYellow='\033[0;93m'      # Yellow
IBlue='\033[0;94m'        # Blue
IPurple='\033[0;95m'      # Purple
ICyan='\033[0;96m'        # Cyan
IWhite='\033[0;97m'       # White

# Bold High Intensity
BIBlack='\033[1;90m'      # Black
BIRed='\033[1;91m'        # Red
BIGreen='\033[1;92m'      # Green
BIYellow='\033[1;93m'     # Yellow
BIBlue='\033[1;94m'       # Blue
BIPurple='\033[1;95m'     # Purple
BICyan='\033[1;96m'       # Cyan
BIWhite='\033[1;97m'      # White

# High Intensity backgrounds
On_IBlack='\033[0;100m'   # Black
On_IRed='\033[0;101m'     # Red
On_IGreen='\033[0;102m'   # Green
On_IYellow='\033[0;103m'  # Yellow
On_IBlue='\033[0;104m'    # Blue
On_IPurple='\033[0;105m'  # Purple
On_ICyan='\033[0;106m'    # Cyan
On_IWhite='\033[0;107m'   # White


printfc () {
  status=$1
  topic=$2
  message=$3

  topic_fmt="${Blue}${On_Yellow}[${topic}]${Color_Off}"

  case $status in
  OK)
    status_fmt="${BGreen}${On_Black}[DONE   ]${Color_Off}"
  ;;
  RUN)
    status_fmt="${BBlue}${On_Black}[RUNNING]${Color_Off}"
  ;;
  FAILED)
    status_fmt="${BRed}${On_Black}[FAILED ]${Color_Off}"
  ;;
  *)
    status_fmt="${BRed}${On_Black}[unknown status $status]${Color_Off}"
esac

message_fmt="${White}${On_Cyan}$message${Color_Off}"
printf "${status_fmt}${topic_fmt}${message_fmt}\n"

}"###;
    ret.to_string()
}
