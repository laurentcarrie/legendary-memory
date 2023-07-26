#!/usr/bin/env sh

set -e
#set -x

pdf=alanis-morissette/ironic.pdf
mkdir -p $(dirname $pdf)

../bin/metapost_of_music_sheet.exe resources/alanis-morissette-ironic.yml

( test -f $pdf  || ( echo "$pdf not found" && false ))

echo DONE
