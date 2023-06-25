#!/bin/bash

set -e
#set -x

echo "make ml of sh"

here=$(dirname $(realpath $0))

new_mlfile=$here/sh_code.ml.tmp
mlfile=$here/sh_code.ml

generate_one_mlfile() {
  cd $here/shfiles

  name=$1

  shfile=$here/shfiles/$1.sh
  tmp_mlfile=$here/shfiles/$1.ml

  test -f $shfile

  sh_data=$(cat $shfile)

  cat <<-EOF >$tmp_mlfile
let make_$name : string =
  {whatever|$sh_data
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
    echo "code has changed for $mlfile"
    echo "$new_data" >$mlfile
    echo "$old_data" >${mlfile}.old
    exit 1
  else
    echo "code has not changed"
  fi


}

preamble

what="make_lytex make_mpost make_pdf make_wav make_clean"

for s in $what ; do
  generate_one_mlfile $s
done
conclusion
