nginx configuration
-------------------

more information about nginx wedav there :
https://github.com/EvilVir/Nginx-Autoindex/blob/master/README.md

you may need to install a few packages, on top of nginx distribution, like :

```
sudo apt-get install libnginx-mod-http-xslt-filter
sudo apt-get install libnginx-mod-http-dav-ext
```

run the `install.sh` script. This will enable the `songbook.conf` module, and copy the scripts
in a suitable location.



test it
-------
point your browser to `<your server>/scripts/cpuinfo.sh` should run the script on the server and return some information

locations made available
------------------------
- location `/`
  this points to the `delivery` directory of the build tree. We will find there the pdf outputs
- location `/song-data`
  this points to the data directory, where the `song.json`, `tex` and `ly` source files are
- location `/scripts`
  this points to the scripts directory, and allow to trigger a script from the web browser
  :warning: anybody could run the script, don't put anything sensitive


scripts made available
-------
- `/scripts/cpuinfo.sh`
- `/scripts/build.sh`
  builds the outputs
