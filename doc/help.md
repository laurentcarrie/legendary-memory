# configure the project
edit the OMakefile, and find 3 variables :
- songs_dir : the directory where songs are found
- books_dir : the directory where books are found
- build_dir : the directory where pdf and wav files will be built
- remote    : see rclone documentation, to upload your pdf files to your google drive, to share with your band

# build the binary
    omake build

will run `cargo build --release` in the `song` binary, under rust/src/target/release

# generate the tree structure

    omake tree

this requires that the binary is built.
It will generate the omake build tree, under $build_dir

# build the songs

    omake songs

this requires that the tree is built
It will build the songs pdf and wav files, in the $songs_dir directory.

# build the books

    omake books

this requires that the tree is built
It will build the trees

# deleting something in the build tree

If you delete a OMakefile from the build tree, the following call to omake will fail,
so you need to either :

- run `touch <path to the OMakefile>`
- run `rm -rf <the builddir>` and then `omake tree`

# update the date of song

    omake compute-date

will use a digest stored in the song definition, computed with the tex and other files needed to generate the pdf,
and update the date to today if the digest don't match.
This will allow to see the last date of modification in the pdf file.
Unlike the other commands, this modifies the `song.json` in the songs source directory, it does act on the build directory,
and has to be called before the build.

# working on only one song

just run

    ( cd <build / path to your song> && omake pdf )

# clean

    omake clean

# file date

the script `compute-date.sh`

# debug

if a tex file is incorrect, the build of the pdf will be stuck. Look at :

    <build / path to your song>/main.pdf.stdout.log

you will find the `lualatex` error message there


# whole picture

to safely rebuild everything from scratch, you should :

    omake build
    rm -rf < build dir >
    omake tree
    omake pdf
    omake wav
    omake gdrive
