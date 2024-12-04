#!/bin/sh

set -e
set -x

echo $PWD

songsdir=$(realpath $1)
[[ -n $songsdir ]]
[[ -d $songsdir ]]

booksdir=$(realpath $2)
[[ -n $booksdir ]]
[[ -d $booksdir ]]

outdir=$(realpath $3)
[[ -n $outdir ]]
[[ -d $outdir ]] || ( mkdir $outdir )
[[ -d $outdir ]] 
chmod 777 $outdir


help() {
    println "$0 <songs dir> <books dir> <build dir>\n"
}

here=$(dirname $(realpath $0))

docker run -v $songsdir:/songs -v $booksdir:/books -v $outdir:/build:rw songs
