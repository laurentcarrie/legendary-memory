#!/bin/bash

set -e
#set -x

echo "make ml of ly"

here=$(dirname $(realpath $0))

new_mlfile=$here/ly_code.ml.tmp
mlfile=$here/ly_code.ml

generate_one_mlfile() {
  cd $here/lyfiles

  name=$1

  lyfile=$here/lyfiles/$1.ly
  tmp_mlfile=$here/lyfiles/$1.ml

  test -f $lyfile

  ly_data=$(cat $lyfile)

  cat <<-EOF >$tmp_mlfile
let make_$name : string =
  {whatever|
  $ly_data
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
    echo "$old_data" >${mlfile}.old
    exit 1
  else
    echo "code has not changed"
  fi


}

preamble

what="macros"

for s in $what ; do
  generate_one_mlfile $s
done
conclusion
