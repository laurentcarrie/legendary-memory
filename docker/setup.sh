#!/bin/bash

set -e
set -x

here=$(dirname $(realpath $0))
root=$(dirname $here)

fix_codespace() {
sudo apt-get install libyaml-tiny-perl libfile-homedir-perl
}

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
    cp $root/software/fonts/*.ttf $HOME/.local/share/fonts/.
    fc-cache -f -v

    mkdir p $HOME/.fonts
    cp $root/software/fonts/*.ttf $HOME/.fonts

}

all() {
  fix_codespace
  install_rust
  install_fonts
  install_omake
  install_lilypond
}

case $1 in
fonts)
install_fonts
;;
all)
  all
  ;;
*)
  echo "bad option"
  esac
