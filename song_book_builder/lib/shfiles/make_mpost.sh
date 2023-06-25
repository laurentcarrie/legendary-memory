#!/bin/bash

set -e
#set -x

mpost --tex=latex $1 1> $1.mpost.stdout.log 2> $1.mpost.stderr.log
