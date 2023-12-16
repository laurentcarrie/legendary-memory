#!/bin/sh

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
