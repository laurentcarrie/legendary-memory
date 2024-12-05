# work with docker

main reason (for me) to use a docker image is when working with github codespace, that has a very old texlive installation.
you may find plenty of other reasons...

# build the image

    ./docker/build.sh


# run the image

we have 3 mounted directories :

- /songs : the songs

- /books : the books

- /build : where the outputs will be generated

the local corresponding directories should be passed to the run script

 typical example run

    ./docker/run.sh data/songs data/books build

will generate the pdfs in the build directory. It should initially not exist
