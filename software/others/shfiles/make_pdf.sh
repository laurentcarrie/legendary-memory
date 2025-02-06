#!/bin/sh

here=$(dirname $(realpath $0))

source $here/colors.sh

buildroot=$2
html_output=$2

#set -e
#set -x

stats(){
count1=$(find $buildroot -name OMakefile | wc --lines)
count2=$(find $buildroot -name main.pdf | wc --lines)

echo "buildroot is $buildroot<br>"
echo "$count2 / $count1 <br>"
}


workdir=$(dirname $(realpath $1.tex))
pprintworkdir=$(echo $workdir | sed "s#$here##")


#stats
printfc RUN pdf $pprintworkdir $html_output

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

printfc OK pdf $pprintworkdir $html_output

#stats
