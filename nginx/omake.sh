#!/bin/sh
rm -rf progress.html
omake -j 8 -k 1>omake.stdout 2>omake.stderr
