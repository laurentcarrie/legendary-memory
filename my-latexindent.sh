#!/bin/bash

tmpresultfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)

echo 0 > $tmpresultfile
here=$(dirname $realpath $0)

work() {
  file=$1
	latexindent -s -w $file --check
	if ! test "x$?" = "x0" ; then
	  echo "reformat :$file"
	  echo 1 > $tmpresultfile
	fi
  rm $(dirname $file)/*.bak*
  rm $(dirname $file)/*.log
}

git ls-files | while read f ; do
	ext=${f##*.}
	if test "x$ext" = "xtex" ; then
	  work $f
	fi
done

ret=$(cat $tmpresultfile)
if test "x$ret" = "x0"; then
  exit 0
fi

exit 1
