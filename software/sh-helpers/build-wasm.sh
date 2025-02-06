#!/bin/sh

set -e
#set -x

here=$(dirname $(realpath $0))
root=$(dirname $(dirname $here))
build_wasm() {
  what=$1
  root=$(dirname $(dirname $here))
  wasmdir=$(dirname $here)/wasm
	cd $wasmdir/$what && wasm-pack build --target web
	target=$root/static/$what
	mkdir -p $target/js
	cp $wasmdir/$what/static/* $target/.
	cp $wasmdir/$what/pkg/* $target/js/
	cp $wasmdir/$what/pkg/* $target/
}

build_wasm lsdir
