#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))

( cd $here/software && cargo fmt && cargo test && cargo build )

$here/software/target/debug/songs $here/data/songs $here/data/books $here/build

( cd $here/build && omake pdf && echo DONE )
