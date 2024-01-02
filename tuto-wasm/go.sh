#!/bin/sh

set -e
set -x

#cargo install wasm-pack
#cargo new --lib hello-wasm

here=$(dirname $(realpath $0))
project=$1
project_dir=$here/$project

cd $project_dir && wasm-pack build --target web
cd $project_dir && python -m http.server


