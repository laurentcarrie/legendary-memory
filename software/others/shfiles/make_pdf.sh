#!/bin/sh

here=$(dirname $(realpath $0))

source $here/colors.sh

buildroot=$2

#set -e
#set -x


workdir=$(dirname $(realpath $1.tex))
pprintworkdir=$(echo $workdir | sed "s#$here##")

for f in main.tex chords.tex sections.tex preamble.tex ; do
  cp $buildroot/songs/$f $workdir/$f
done

rm -f main.pdf

#stats
printfc RUN pdf $pprintworkdir

i="0"
while [ $i -lt 4 ]
do
lualatex $1.tex 1> $1.pdf.stdout 2> $1.pdf.stderr
test -f main.log
count=$(cat $1.log | grep Rerun | wc -l)
if test "x$count" = "x0" ; then
    break
fi
i=$[$i+1]
done

if test -f main.pdf ; then
  printfc OK pdf $pprintworkdir
else
  printfc FAILED pdf $pprintworkdir
fi
