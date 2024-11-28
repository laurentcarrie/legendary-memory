#!/usr/bin/env sh

#set -e
#set -x

rm -rf *.pdf *.log *.aux mps *.mpx *.dep *.lytex *.wav *.stdout *.stderr *.midi lock
find . -name "*.eps" | while read f ; do echo $f ; rm -rf $(dirname $f) ; done
