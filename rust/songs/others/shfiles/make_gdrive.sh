#!/usr/bin/env sh

set -e
set -x

dirname=$1
test -d $dirname
test -f "$dirname/maroon_5--@--this_love.pdf"

(
  cd $dirname
  rclone sync . mydrive:/zik/songs --differ -
)
