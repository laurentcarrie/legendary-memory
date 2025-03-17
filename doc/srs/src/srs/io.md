### <a id="srs/output"/> pdf output
the tool output will be pdf files

we want to be able to print a songbook, and also to download it on a tablet, and be available without
internet connection

### <a id="srs/wav"/> wav files

whenever a piece of music sheet is present in the song, it will be possible to generate a wav output for
that piece.

### <a name="srs/input"/> input

all inputs are readable text files. It will therefore be possible to put them in a git repo and manage the
life of these files


### <a name="srs/mode"/> two edit modes

there will be two modes : the local and the web mode


### local mode

in local mode, you have access to a machine where you cloned the repo that has the songs, and you also
cloned the repo that has the code of the tool. (currently they are in the same repo)

it requires that you have some computer science knowledge, as you will have to install a few things,
edit the data files and run the tool

### OS

the software will run on ubuntu. There is no requirement that it runs on windows.

### output export

provided correct configuration, it will be possible to export the pdf outputs to a google drive

### web mode

in web mode, you edit the remote files via a web interface, you trigger the generate of the pdf file by
clicking a button on the web interface
