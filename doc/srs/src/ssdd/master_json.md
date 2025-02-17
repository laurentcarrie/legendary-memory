# <a id="master"/> master json

We use serde_json to automate reading from json file to a rust struct, this is documented here :

[rust doc for master file](/legendary-memory/songbook/config/input_model/struct.UserSong.html)


## available section types

The list of valid sections is built at compile time, it is read with serde json from this file :
 [others/texfiles/sections.json](/legendary-memory/others/texfiles/sections.json)

To modify the available section types, just change the file and rebuild.
