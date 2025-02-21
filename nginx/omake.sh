#!/bin/sh
build_id=$1 omake -j 8 -k 1>omake.$build_id.stdout 2>omake.$build_id.stderr
