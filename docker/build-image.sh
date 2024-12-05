#!/bin/sh

set -e
set -x

here=$(dirname $(realpath $0))
rootdir=$(dirname $here)

(
    cd $rootdir
    docker build -t songs -f docker/Dockerfile .
)
