#!/usr/bin/env bash

set -e
#set -x

here=$(dirname $(realpath $0))


builddir=$1
test "x$builddir" != "x"
deliverydir=$2
test "x$deliverydir" != "x"

source $here/colors.sh

find $builddir/songs -name "song-internal.json" | while read json ; do
  songbuilddir=$(cat $json | jq -r ".builddir")
  songpdfname=$(cat $json | jq -r ".pdfname")
  pdffile=$songbuilddir/${songpdfname}.pdf
  test -f $pdffile
  cp $pdffile $deliverydir/.
done

find $builddir/books -name "book.json" | while read json ; do
  bookbuilddir=$(cat $json | jq -r ".builddir")
  bookpdfname=$(cat $json | jq -r ".pdfname")
  pdffile=$bookbuilddir/${bookpdfname}.pdf
  test -f $pdffile
  cp $pdffile $deliverydir/.
done
