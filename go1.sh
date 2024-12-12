#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

if [[ true ]] ; then
  type=""
  exe=$here/software/target/debug/songs
else
  type="--release"
  exe=$here/software/target/release/songs
fi



( cd software && cargo fmt && cargo build $type && cargo test $type )
#cargo install --path $here/software || ( echo "build FAILED" && false )


artist=alannah_myles
song=black_velvet

rm -rf $here/data2
mkdir -p $here/data2/songs
mkdir -p $here/data2/books
mkdir -p $here/data2/songs/$artist
cp -R $here/data/songs/$artist/$song $here/data2/songs/$artist/$song
cp -R $here/data/books/dummy.json $here/data2/books/.

$exe $here/data2/songs $here/data2/books $here/build

echo "SUCCESS"


( cd $here/build/songs/$artist/$song && omake pdf && echo DONE ) || ( echo FAILED )
( cd $here/build/books/concert_du_13_fevrier && omake pdf && echo DONE) || ( echo FAILED )
