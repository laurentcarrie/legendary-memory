#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

( cd software && cargo fmt )
cargo install --path $here/software || ( echo "build FAILED" && false )

#rm -rf $here/build

songs $here/data/songs $here/data/books $here/build

( cd $here/build/songs/$1 && omake pdf && echo DONE ) || ( echo FAILED )
