#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))
root=$(dirname $(dirname $here))
bash $here/check-json.sh
bash $here/add-missing-lyrics.sh

( cd $root/software && cargo fmt && cargo test && cargo build )

songsdir=$root/data/songs
booksdir=$root/data/books
builddir=$root/build

$root/software/target/debug/songs $songsdir $booksdir $builddir

( cd $builddir && omake pdf -j 8 && echo DONE )
