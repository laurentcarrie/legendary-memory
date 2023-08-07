#!/bin/bash

set -e
set -x

echo "make ml of mp"

here=$(dirname $(realpath $0))

new_mlfile=$here/mp_code.ml.tmp
mlfile=$here/mp_code.ml

generate_one_mlfile() {
  cd $here/mpfiles
  set -e
  set -x

  name=$1

  mpfile=$here/mpfiles/$1.mp
  tmp_mlfile=$here/mpfiles/$1.ml

  test -f $mpfile

  mp_data=$(cat $mpfile)

  cat <<-EOF >$tmp_mlfile
let make_$name : string =
  {whatever|
  $mp_data
  |whatever}

EOF

  cat $tmp_mlfile >> $new_mlfile
  rm $tmp_mlfile

}

ret=0

preamble() {
  rm -f $new_mlfile
  if test -f $mlfile; then
    old_data=$(cat $mlfile)
  else
    old_data=""
  fi
}

conclusion() {
  new_data=$(cat $new_mlfile)
  rm -f $new_mlfile

  if test "x$new_data" != "x$old_data"; then
    echo "code has changed"
    echo "$new_data" >$mlfile
    exit 1
  else
    echo "code has not changed"
  fi

}

preamble
for s in flat sharp ; do
  echo $s
  generate_one_mlfile $s
done
conclusion
