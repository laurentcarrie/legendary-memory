# inputs and outputs requirements

| Requirement                    | Description                                |
|--------------------------------|--------------------------------------------|
| [pdf output](io.md#srs/output) | the tool output will be pdf files          |
| [wav output](io.md#srs/wav)    | the tool will output wav files when needed |
| [input](#srs/input)            | the input are text files                   |


# export requirements

| Requirement                    | Description            |
|--------------------------------|------------------------|
| [delivery dir](#srs/delivery-dir)| all export files (pdf and wav) will be available in one dir |
| [google drive](#srs/gdrive) | export to google drive |


# dependencies requirements
| Requirement      | Description                                            |
|------------------|--------------------------------------------------------|
| [os](#srs/os) | OS and required tools |


# interface requirements : two user modes
| Requirement                                     | Description                 |
|-------------------------------------------------|-----------------------------|
| [text editor](<a name="srs/text-editor-mode"/>) | text editor mode            |
| [web mode](local.md#local)                     | web browser mode |


# dependencies requirements
| Requirement              | Description                                            |
|--------------------------|--------------------------------------------------------|
| [os](<a name="srs/os"/>) | OS and required tools |


# rendering requirements :

what to find in the pdf output

| Requirement                                            | Description                                                  |
|--------------------------------------------------------|--------------------------------------------------------------|
| [sections](render.md#sections)                         | a song is structured as sections                             |
| [refs](<a name="srs/references"/>)                     | a section can be a reference to another one                  |
| [musicsheet](render.md#musicsnippet)                   | insertion of music sheet snippet                             |
| [tempo](render.md#tempo)                               | show song tempo                                              |
| [time signature](render.md#time-signature)             | show time signature                                          |
| [color](<a name="srs/color"/>)                         | coloring of sections                                         |
| [bar numbering](<a name="srs/barnumber"/>)             | show numbering of bars                                       |
| [time](<a name="srs/time"/>)                           | show time on bars                                            |
| [book](<a name="srs/book"/>)                           | definition of book                                           |
| [last modified time](<a name="srs/lastmodifiedtime"/>) | the last modified time of a song will be rendered            |
| [lyrics](<a name="srs/lyrics"/>)                       | lyrics will be rendered, synced with the sections            |
| [coherence](render.md#coherence)                       | tempo, signature and bar numbers will be coherent in the doc |
| [table](render.md#grid)                                | the section chords will be rendered as tables                |
| [chord symbol](render.md#chord-symbol)                 | the chords in a table will be rendered in a standard way     |
| [chords per bar](render.md#chords-per-bar)             | render up to 2 chords per bar                                |
| [line repeat](render.md#line-repeat)                   | show line repeats to make rendering smaller                  |
| [text rendering](render.md#text-rendering)  | text rendering will have a lot of features                   |







# <a id="srs/wav"/> wav files

whenever a piece of music sheet is present in the song, it will be possible to generate a wav output for
that piece.

# <a id="srs/input"/> input

all inputs are readable text files. It will therefore be possible to put them in a git repo and manage the
life of these files


# <a name="srs/mode"/> two edit modes

there will be two modes : the local and the web mode




## output export

provided correct configuration, it will be possible to export the pdf outputs to a google drive

## web mode

in web mode, you edit the remote files via a web interface, you trigger the generate of the pdf file by
clicking a button on the web interface

## <a id="srs/os"/> Operating System

the software will run on standard ubuntu. There is no requirement that it runs on windows.


## <a id="srs/delivery-dir"/> Delivery Directory

All output files will be available in one directory.

## <a id="srs/gdrive"/> google drive

A command will allow to upload all pdf files to a google drive location,
if correct credentials are provided
