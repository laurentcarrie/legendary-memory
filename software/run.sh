#!/bin/sh

set -e
set -x

# meant to work inside the docker container
songs /songs /books /build
( cd /build &&  omake delivery -j 8 )
