#!/bin/bash

set -e
set -x

install_rust() {
    curl https://sh.rustup.rs -sSf -o rustup.sh
    bash rustup.sh -y
}

install_lilypond() {
    curl -L https://gitlab.com/lilypond/lilypond/-/releases/v2.24.4/downloads/lilypond-2.24.4-linux-x86_64.tar.gz -o lilypond.tar.gz
    tar xvzf lilypond.tar.gz
    rm lilypond.tar.gz
    rm -rf $HOME/lilypond
    mkdir $HOME/lilypond
    mv lilypond-2.24.4 $HOME/lilypond
    echo export PATH="\$HOME/lilypond/lilypond-2.24.4/bin:\$PATH" >> $HOME/.bashrc
}

install_omake() {
    sudo apt-get update
    sudo apt-get install omake -y
}

install_fluidsynth() {
    sudo apt-get update
    sudo apt-get install fluidsynth -y
}


install_fonts() {
    mkdir -p $HOME/.local/share/fonts
    cp software/fonts/* $HOME/.local/share/fonts/.
    fc-cache -f -v
}

install_rust
install_fonts
install_omake
install_lilypond
