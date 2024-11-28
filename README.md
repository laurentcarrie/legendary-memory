# What is this ?

This is a tool to build pdf and wav files, in a form suitable ( for me ) to play in a pop/rock/jazz band.
This needs to be :

- compact

partitions tend to be very long, pages long, but pop music is made of repetetions of verses, bridge, chorus, and I want
that to stand in one A4 page
![alt text](doc1.png)

- table oriented

We want to see grid of chords.
![alt text](doc2.png)

- versioned in github, as text files. We don't want a word or other format

last modified date will automatically be updated.

- normalized

We want to have all music sheets with the same look

- support of real music notation

We want to able to import music notation, we choose lilypond music notation for that

![alt text](doc3.png)


- support of midi and wav export

We will use lilypond and synth for that.

- support for all latex features, in case we want to enhance the look of the lyrics. I have to admit that,
not being the singer in the band, I did not put much effort on that.
But this is latex, you can put anything you like.

![alt text](doc4.png)

- support for books ( set list )

a book is a list of songs. You usually work on a set list, and you don't want to browse your google drive to 
find each song when you rehearse. We want to have all of them in one file.
