# Overview

There are many ways to achive that. We could use a word processing tool, such as Microsoft Word, latex,
lilypond, musescore or even sketch with a pencil on a paper and take a picture.

but none of these respect our requirements, so this is our design :

## text : latex
textual information, such as lyrics, comments, annotations,... are captured in latex files. This way we have
the formating flexibility we want.

We could have done everything with latex, except from the music sheet, by writing a new latex class. But
choose to limit latex, for the user, to the capture of text.

## music sheet : [lilypond](https://lilypond.org/)
Music sheet snippets are captured in lilypond files. We will be able to output the partitions of solos, bridges,
chord diagrams, ... and insert them in the pdf output file. This will also allow to export wav files, because
lilypond has a midi export.

there are other tools to do that, but lilypond is way superior, at the cost of some complexity. We think it is
worth the cost.

## [master file](master_json.md#master)
A master file will contain the description of the sections, the tempo, and all other informations.
This master file will have a fixed name : ``song.json``.

## master tex file
A master tex file will contain the latex information. This file is named ``body.tex``.
This is not the ``main.tex`` file, that contains the
document class, this file is generated.

## book
A book is only a list of songs, with a book title. It is defined in a [json file](book.md#top)

## file organization
Each song is contained in one directory. It will have :
1. `song.json` : the master file
2. `body.tex` : the master tex file
3. `lyrics` directory : contains the lyrics of the song, one tex file per section
4. additional regular tex files. You just import them as you would do
5. additional lilypond files (.ly extension).

The information to name the pdf file are inside the ``song.json`` master file, but, for sake of readability,
we will use :

    songdir root
       +--- artist 1
               +--- song 1
                      +--- song.json
                      +--- body.tex
                      +--- ... other tex files if any...
                      +--- ... lilypond files if any ....
                      +--- lyrics
                             +--- lyrics tex fils
               +--- song i
               +--- song N
                      .... subtree of song 2
       +--- artist i
       +--- artist N
               +--- a song
                      ... files
               +--- another song
                      ... files



    bookdir root
        +--- book1.json
        +--- ...
        +--- bookN.json

## file tree
all songs will be under a root directory : the ``songdir`` directory.

all books will be under a root directory : the ``bookdir`` directory. Each json file in this directory defines a book.


## omake

the omake build tool [omake](http://projects.camlcity.org/projects/omake.html), not to be confused with IBM omake tool,
is a make tool that looks like make, but is actually much superior :
1. it does not have the dependency issue, see [recursive make considered harmful](https://accu.org/journals/overload/14/71/miller_2004/), which prevents from efficiently use make in big projects
2. dependencies are built on checksum of files, not on dates, so it does not matter if you regenerate files before running a build
3. a lot of other stuffs... not discussed here

we want a build system because building a pdf or a wav file can be long, and we are going to iterate a lot when capturing a new song,
especially with the lilypond files.


## the tools
we have three tools, written in rust, in this project :
1. ``songbook`` : this is a code generator
2. for web only : ``songbook-server`` [see server](web/server.md#top)
2. for web only : ``songbook-client`` [see client](web/client.md#top)


## workflow

this is :

1. run ``songbook <songdir> <bookdir> <builddir>``
2. ``( cd builddir && omake )``

the songbook tool will
1. read all the songs from the ``songdir`` tree
2. read all the books from the ``bookdir`` tree,
3. generate the OMakeroot and OMakefile in the ``builddir`` tree
4. genere some ressource files at the root of the ``builddir`` tree

you can then run omake.


## code generation

[see code generation section](generate.md#overview)


### lilypond files
