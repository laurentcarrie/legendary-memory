#!/bin/sh

set -e
#set -x

here=$(dirname $(realpath $0))
. $here/software/others/shfiles/colors.sh


nb_workers=10

#if false ; then
if test  "x$1" = "x" ; then
	songdir=$here/data/songs
	bookdir=$here/data/books
	builddir=$here/build
else
  path=$1
	title=$(basename $path)
	path=$(dirname $path)
	author=$(basename $path)
	printf "${BRed}${author}${Color_Off} ${BYellow}${title}${Color_Off}\n"
	#song="bowie/modern_love"
	#song="dolly/je_n_veux_pas_rester_sage"
	songdir=$here/data/songs/$author/$title
	bookdir=$here/data/empty
	builddir=$here/build
	rm $(find $builddir -name ".checksum_ok" ) || true
fi



set -e
( cd $here/software && cargo fmt && cargo +nightly build --bin songbook-demo )

tmux kill-session -t build || true
tmux new -d -s build
tmux split-window -t build
tmux split-window -h -t build
tmux split-window -t build

rm -f $here/software/*.log

tmux send-keys -t build:0.0 "cd $here/software" C-m
tmux send-keys -t build:0.1 "cd $here/software" C-m
tmux send-keys -t build:0.2 "cd $here/software" C-m

#tmux send-keys -t build:0.0 "cargo +nightly build --bin songbook-demo" C-m
tmux send-keys -t build:0.0 "./target/debug/songbook-demo --nb-workers $nb_workers $songdir $bookdir $builddir" C-m

tmux send-keys -t build:0.1 "while true ; do tail -f songbook.log ; sleep 3 ; done" C-m
tmux send-keys -t build:0.2 "while true ; do tail -f lualatex.log ; sleep 3 ; done" C-m
tmux send-keys -t build:0.3 top C-m



tmux attach -t build:0.0
