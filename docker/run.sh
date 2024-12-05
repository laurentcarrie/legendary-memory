#!/bin/bash

set -e
set -x

songsdir=$(realpath $1) ; shift
[[ -n $songsdir ]]
[[ -d $songsdir ]]

booksdir=$(realpath $1) ; shift
[[ -n $booksdir ]]
[[ -d $booksdir ]]

outdir=$(realpath $1) ; shift
[[ -n $outdir ]]
[[ -d $outdir ]] || ( mkdir $outdir )
[[ -d $outdir ]]
chmod 777 $outdir


help() {
    println "$0 <songs dir> <books dir> <build dir>\n"
}

here=$(dirname $(realpath $0))

docker run \
    -v $songsdir:/songs \
    -v $booksdir:/books \
    -v $outdir:/build:rw \
    -v $HOME/.cargo/bin:/songbin \
    songs
