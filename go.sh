#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

#cargo install --path $here/software

export songdir=$here/data/songs
export bookdir=$here/data/books
export builddir=$here/build

#export songdir=$here/data2/songs
#export bookdir=$here/data2/books
#export builddir=$here/build


rm -rf build/songs/telephone
songbook $songdir $bookdir $builddir

#( cd $builddir && omake -j 8 delivery/the_police--@--every_breath_you_take.pdf) || bash $builddir/omake.sh
#( cd $builddir && omake -j 8 delivery/bashung--@--la_nuit_je_mens.pdf) || bash $builddir/omake.sh
( cd $builddir && omake -j 8 delivery/telephone--@--au_coeur_de_la_nuit.pdf) || bash $builddir/omake.sh
#( cd $builddir && omake -j 8 ) || bash $builddir/omake.sh
