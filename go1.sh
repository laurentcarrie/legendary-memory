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


#rm -rf $here/build

#songs $here/data/songs $here/data/books $here/build
rm -rf $here/data2
mkdir -p $here/data2/songs
mkdir -p $here/data2/books
cp -R $here/data/songs/amy_winehouse $here/data2/songs/.
cp -R $here/data/books/dummy.json $here/data2/books/.

$exe $here/data2/songs $here/data2/books $here/build

echo "SUCCESS"

song=amy_winehouse/you_know_i_m_no_good

( cd $here/build/songs/$song && omake pdf && echo DONE ) || ( echo FAILED )
