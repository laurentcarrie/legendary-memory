here=$(dirname $(realpath $0))
export songdir=/home/laurent/work/legendary-memory/data/songs
export bookdir=/home/laurent/work/legendary-memory/data/books
export builddir=$here/build
export softwaredir=/home/laurent/work/legendary-memory/software
#export song=bashung/la_nuit_je_mens


neomake plan -n all -a "args.blah=x" | neomake execute -w4
#neomake plan -n a | neomake execute