#!/bin/bash

set -e
set -x

echo "make rs of sh"

here=$(dirname $(realpath $0))

mkdir -p $here/src/generated

generate_one_outfile() {
  name=$1
  shsrcdir=$2
  extension=$3
  outfile_tmp=$4
  outfile=$5

  shfile=$shsrcdir/$name.$extension
  printf "$shfile\n"

  test -f $shfile

  sh_data=$(cat $shfile)

  cat <<-EOF >>$outfile_tmp
pub fn make_$name() -> String {
  let ret = r###"$sh_data"### ;
  ret.to_string()
}
EOF


}

ret=0

preamble() {
  outfile_tmp=$1
  outfile=$2
  if test -f $outfile; then
    old_data=$(cat $outfile)
  else
    old_data=""
  fi
  rm -f $outfile_tmp
}

conclusion() {
  outfile_tmp=$1
  outfile=$2
  if test -f $outfile ; then
    old_data=$(cat $outfile)
  else
    old_data=xxx
  fi
  cargo fmt -- $outfile_tmp
  new_data=$(cat $outfile_tmp)
  rm $outfile_tmp
  if test "x$new_data" != "x$old_data"; then
    echo "code has changed for $outfile"
    echo "$new_data" >$outfile
    echo "$old_data" >${outfile}.old
    exit 1
  else
    echo "code has not changed"
  fi


}

what="make_lytex make_mpost make_pdf make_wav make_clean"
shsrcdir=$here/others/shfiles
extension=sh
outfile_tmp=$here/src/generated/sh_code.rs.tmp
outfile=$here/src/generated/sh_code.rs
#preamble $outfile_tmp $outfile
for s in $what ; do
  generate_one_outfile $s $shsrcdir $extension $outfile_tmp $outfile
done
conclusion $outfile_tmp $outfile


what="macros"
shsrcdir=$here/others/lyfiles
extension=ly
outfile_tmp=$here/src/generated/ly_code.rs.tmp
outfile=$here/src/generated/ly_code.rs
#preamble $outfile_tmp $outfile
for s in $what ; do
  generate_one_outfile $s $shsrcdir $extension $outfile_tmp $outfile
done
conclusion $outfile_tmp $outfile

what="preamble"
shsrcdir=$here/others/texfiles
extension=tex
outfile_tmp=$here/src/generated/tex_code.rs.tmp
outfile=$here/src/generated/tex_code.rs
#preamble $outfile_tmp $outfile
for s in $what ; do
  generate_one_outfile $s $shsrcdir $extension $outfile_tmp $outfile
done
conclusion $outfile_tmp $outfile


what="flat sharp seven minor major_seven draw_bati glyph_of_chord draw_row draw_chord"
shsrcdir=$here/others/mpfiles
extension=mp
outfile_tmp=$here/src/generated/mp_code.rs.tmp
outfile=$here/src/generated/mp_code.rs
#preamble $outfile_tmp $outfile
for s in $what ; do
  generate_one_outfile $s $shsrcdir $extension $outfile_tmp $outfile
done
conclusion $outfile_tmp $outfile
