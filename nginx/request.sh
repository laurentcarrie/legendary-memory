#!/bin/sh
## -*- coding: utf-8 -*-
##NAME=`"cpuinfo"`

if test "x$1" != "x" ; then
  COMMAND=$1
elif test "x$QUERY_STRING" != "x" ; then
  COMMAND=$QUERY_STRING
else
  echo "no argument"
  exit 1
fi

if test "x$2" != "x" ; then
  MIME="application/json"
else
  MIME=$2
fi

here=$(dirname $(realpath $0))
request=$(echo $COMMAND | sed "s/^request=//")
work() {
  echo "Content-type:$MIME/json\r\n"
  $here/songbook-client $request 2>/dev/null | jq "."
}

debug() {
  echo "Content-type:text/text\r\n"
  tmpfile=$(mktemp)
  echo $request | base64 --decode
  echo $request | base64 --decode | jq "."
  echo "BEGIN STDOUT"
  $here/songbook-client $request 2>$tmpfile
  echo "END STDOUT"
  echo "BEGIN STDERR"
  cat $tmpfile
  echo "END STDERR"
}

#debug
work
