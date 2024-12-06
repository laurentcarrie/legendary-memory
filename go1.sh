#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

( cd software && cargo fmt )
cargo install --path $here/software || ( echo "build FAILED" && false )

#rm -rf $here/build

songs $here/data/songs $here/data/books $here/build

echo "SUCCESS"

song=amy_winehouse/you_know_i_m_no_good

( cd $here/build/songs/$song && omake pdf && echo DONE ) || ( echo FAILED )
