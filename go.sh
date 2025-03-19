#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

cargo install --path $here/software

songbook $songdir $bookdir $builddir

#( cd $builddir && omake -j 8 delivery/the_police--@--every_breath_you_take.pdf) || bash $builddir/omake.sh
( cd $builddir && omake -j 8 ) || bash $builddir/omake.sh
