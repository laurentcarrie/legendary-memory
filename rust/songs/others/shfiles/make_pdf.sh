#!/bin/sh

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

printfc OK pdf $pprintworkdir