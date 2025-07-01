#!/bin/bash

here=$(dirname $(realpath $0))
export songdir=
export bookdir=/home/laurent/work/legendary-memory/data/empty
export builddir=/home/laurent/work/legendary-memory/build

neomake plan -n all -a "args.blah=x" | neomake execute -w4
