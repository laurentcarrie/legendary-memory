#!/bin/bash

set -e

name=$1
test "x$1" != "x"

while true ;  do
  candidates=$(ps -C $name | tail -n +2 | sed -r 's/ *(\w+?).*/\1/' )
#  echo "candidates : '$candidates'"
  if test "x$candidates" = "x" ; then
#    echo "break"
    break
  fi
  for pid in $candidates ; do
#    echo $pid
#    echo "found $candidate"
    kill -9 $pid
  done
done
#echo "done"
