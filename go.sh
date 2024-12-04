#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

cargo install --path $here/software

songs $here/data/songs $here/data/books $here/build

( cd $here/build && omake delivery && echo DONE )
