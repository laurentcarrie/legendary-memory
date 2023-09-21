#!/bin/sh

set -e
set -x
set -o pipefail

here=$(dirname $(realpath $0))
exe=${here}/metapost_of_music_sheet/_build/default/bin/metapost_of_music_sheet.exe

echo "" >$here/test.txt

mkdir -p $here/tmp
rm -rf $here/tmp/*

f_all() {
  (
    cd $here/tmp
    rm -f stdout.txt
    rm -f stderr.txt
    find $here -name song.yml | grep "$1" | while read f; do
      echo $f
      ($exe $f 1>>stdout.txt 2>>stderr.txt && echo SUCCESS) || echo FAILED
    done

    #$exe $input | tee $here/test.txt
  )
}

f_one() {
  (
    input=$(realpath $1)
    echo $input
    cd $here/tmp
    rm -f stdout.txt
    rm -f stderr.txt
    echo $exe
    ($exe ${input}/song.yml && echo SUCCESS) || echo FAILED
  )
}

case $1 in
  all)
    shift
    f_all $1
    ;;
  one)
    shift
    f_one $1
    ;;
  *)
    echo "bad command"
esac

echo DONE
