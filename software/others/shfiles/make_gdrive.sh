#!/usr/bin/env sh

set -e
set -x

dirname=$2
remote=$1

ls $dirname | while read f ; do
  case "$f" in
      *.pdf) true ;;
      *) echo "this is not a pdf file : $f" ; exit 1;;
  esac
done


(
  cd $dirname
  rclone sync . mydrive:$remote --verbose
)
