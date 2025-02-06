#!/bin/sh

set -e
#set -x

here=$(dirname $(realpath $0))
root=$(dirname $(dirname $here))
songsdir=$root/data/songs
booksdir=$root/data/books
builddir=$root/build

bash $here/check-json.sh $songsdir
bash $here/add-missing-lyrics.sh $songsdir
#bash $here/check-existence-of-files.sh $songsdir

( cd $root/software && cargo fmt && cargo test && cargo build )

$root/software/target/debug/songbook $songsdir $booksdir $builddir





f() {
    ( cd $builddir && omake all -j 8 && echo DONE )
    tar cvzf delivery.tar.gz $builddir/delivery
    #dedix-put delivery.tar.gz
    aws s3 cp delivery.tar.gz s3://dsaa-cph-ai-s3-dev/laurent_carrie/delivery.tar.gz --profile dev
}

g() {
    author=depeche_mode
    song=enjoy_the_silence
    ( cd $builddir/songs/$author/$song && omake pdf && echo DONE )
#    aws s3 cp $builddir/songs/$author/$song/main.pdf s3://dsaa-cph-ai-s3-dev/laurent_carrie/xxx --profile dev
}

f
