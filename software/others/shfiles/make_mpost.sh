#!/bin/bash

set -e
#set -x

here=$(dirname $(realpath $0))
source $here/colors.sh

workdir=$(dirname $(realpath $1.mpost))
pprintworkdir=$(echo $workdir | sed "s#$here##")

printfc RUN mpost $pprintworkdir

mpost --tex=latex $1 1> $1.mpost.stdout 2> $1.mpost.stderr

printfc OK mpost $pprintworkdir
