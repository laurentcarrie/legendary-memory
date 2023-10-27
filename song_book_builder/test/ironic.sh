#!/usr/bin/env sh

set -e
#set -x

pdf=alanis-morissette/ironic.pdf
mkdir -p $(dirname $pdf)

../bin/song_book_builder.exe resources/alanis-morissette-ironic.yml

test -f $pdf  || ( echo "$pdf not found" && false )
echo FOUND : $pdf

count=$(ls $(dirname $pdf) | wc --lines )
echo $count

test $count -eq 1  || ( echo "more than one file : $count" && false )


echo DONE
