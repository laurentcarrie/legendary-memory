#!/bin/sh

set -eo pipefail
set -x

here=$(dirname $(realpath $0))
# the root of the project
root=$(dirname $here)
songdir=$(dirname $here)/data/songs
bookdir=$(dirname $here)/data/books
wwwroot=/var/www/songbook

build_songbook() {
  cargo install --path $(dirname $here)/software
#  sudo mkdir -p $wwwroot/scripts
  sudo cp ~/.cargo/bin/songbook-client $wwwroot/scripts/.
  sudo cp ~/.cargo/bin/songbook-server $wwwroot/scripts/.
  #sudo cp ~/.cargo/bin/songbook $wwwroot/scripts/.
}


build_wasm() {
  what=$1
  wasmdir=$root/software/leptos/$what
	cd $wasmdir
	trunk build --release --public-url /leptos/$what
	targetdir=$wwwroot/leptos/$what
  mkdir -p $targetdir
  cp -R dist/* $targetdir/.
  sudo chown www-data:www-data -R $targetdir

}
make_nginx_conf() {
  sudo cp $here/songbook.conf /etc/nginx/sites-available
  (
    cd /etc/nginx/sites-enabled &&
    sudo rm -f songbook.conf &&
    sudo ln -s /etc/nginx/sites-available/songbook.conf &&
    sudo rm -f default
    )
}

install_songbook_server_service_as_root() {
   rm -f $HOME/.config/systemd/user/songbook.service
   systemctl --user daemon-reload
   sudo cp $here/songbook.service /etc/systemd/system/.
   sudo mkdir -p /var/log/songbook
   sudo systemctl daemon-reload
   sudo service songbook restart
   # test
   bash $here/request.sh $(echo "{\"choice\":{\"ItemHealthCheck\":null}})" | base64 )
}

install_songbook_server_service_as_user() {
   #sudo cp $here/songbook.service /etc/systemd/system/.
   #sudo cp $here/songbook.service /etc/systemd/system/.
   mkdir -p $HOME/.config/systemd/user
   cp $here/songbook.service $HOME/.config/systemd/user/.
   sudo mkdir -p /var/log/songbook
   sudo chown www-data:www-data /var/log/songbook
   sudo rm -f /etc/systemd/system/songbook.service
   systemctl --user enable songbook.service
   systemctl --user stop songbook.service
   systemctl --user start songbook.service
   sudo systemctl daemon-reload
   systemctl --user daemon-reload
   # test
   bash $here/request.sh $(echo "{\"choice\":{\"ItemHealthCheck\":null}}" | base64 )
}

install_packages(){
  sudo apt-get install -y \
      nginx \
      nginx-common \
      libnginx-mod-http-dav-ext \
      libnginx-mod-http-xslt-filter
}

make_www_tree() {
  # build the the file tree that will be available to nginx, we will have :
  # the files for nginx
  # the build files
  # a symbolic link to the song sources

  # root
  sudo rm -rf $wwwroot
  sudo mkdir $wwwroot
  sudo cp $here/autoindex.xslt $wwwroot/.
  sudo cp $here/index.html $wwwroot/.
  sudo cp $here/xxx.html $wwwroot/.
  sudo cp $here/my-ace.js $wwwroot/.

  # scripts
  sudo mkdir -p $wwwroot/scripts
  sudo cp $here/*.sh $wwwroot/scripts/.
  sudo chmod +x $wwwroot/scripts/*.sh

  # webdav
  sudo mkdir -p $wwwroot/client_temp

  # build root
  sudo mkdir $wwwroot/output

  # root for wasm static js files
  sudo mkdir $wwwroot/static

  # source for songs and books
  sudo mkdir -p $wwwroot/input
  sudo cp -R $songdir $wwwroot/input/songs
  sudo cp -R $bookdir $wwwroot/input/books

  sudo chown -R www-data $wwwroot
  sudo chgrp -R www-data $wwwroot
  find $wwwroot -type d | while read f ; do sudo chmod go+w $f ; done
}

restart_nginx() {
  sudo service nginx restart
   #|| sudo cat /var/log/nginx/error.log
  # sudo service nginx status
}



install_packages
make_www_tree
build_songbook
for w in build_progress  source_tree ; do
  build_wasm $w
done
make_nginx_conf
restart_nginx
#install_songbook_server_service_as_root
install_songbook_server_service_as_user


cp -R /home/laurent/work/ace-builds/src-min-noconflict $wwwroot
cp -R /home/laurent/work/ace-builds/src-noconflict $wwwroot


echo "DONE"
