#!/usr/bin/env bash

# adapt this path to your configuration
#texmf_dir=~/texmf
#this is for github codespace
#texmf_dir=/usr/local/texlive/2024/texmf-local
texmf_dir=/usr/local/texlive/2024/texmf-dist

if wget http://mirrors.ctan.org/install/macros/latex/contrib/$1.tds.zip ; then
    unzip -d $texmf_dir $1.tds.zip
    rm $1.tds.zip
    echo installed via tds
elif wget ftp://sunsite.icm.edu.pl/pub/CTAN/systems/texlive/tlnet/archive/$1.tar.xz ; then    
    mv $1.tar.xz $texmf_dir/.
    ( cd $texmf_dir ; pwd ; tar xvf $1.tar.xz ; rm $1.tar.xz )
    echo installed from texlive archive
else
    wget http://mirror.ctan.org/macros/latex/contrib/$1.zip
    unzip $1.zip
    cd $1/
    tex $1.ins
    ctanify *.ins *.dtx
    tar -xzf $1.tar.gz
    unzip -d $texmf_dir $1.tds.zip
    cd ..
    rm -rf $1 $1.zip
    echo "installed via *.ins *.dtx"
fi
test -e ~/texmf/ls-R && texhash $texmf_dir
