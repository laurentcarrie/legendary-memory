# requirements on rendering the pdf output

## <a id="tempo"/> tempo

the tempo will be shown, in BPM. There is one tempo for the whole song, if the real song has different tempos,
this will not be rendered


## <a id="sections"/> sections

what we call a section is what you naturally see in a song : verse, chorus, pre-chorus, bridge, intro, ...

1. it will be possible to define new sections
2. sections will be associated with colors
3. each section has associated lyrics


## <a id="time-signature"/> time signature

time signature will be shown in the output. There will be one time signature for the whole song,
except if a bar is marked with another time signature

## <a id="musicsnippet"/> music sheet snippet

it will be possible to have music sheet snippets in the rendered output pdf file.
This way we can show the solos, riffs, gimmicks....

## <a id="coherence"/> coherence

the sections in the grid rendering will be in coherence with the sections in the lyrics.
This mean that if we have, for instance, a song with verse-1, verse-2, pre-chorus-1, chorus-1, we will
find these sections in both rendering

## <a id="grid"/> grid

the chords will be shown as a grid, with 4 columns, one grid per section

## <a id="chord-symbol"/> chords symbol

the chords will be shown with the A...G symbols ( not Do Ré Mi...).

- the minus symbol will show the minor
- 7 for 7 chords
- flat and sharp
- m7
- sus chords
- 5 chords
- 7M will be show as a triangle
- diminished chords as small circle

the size of a cell for a chord will not change, this means that a ``C`` chord rendering will take as
much width as a ``C#m7``, so all cells have same size



## <a id="chords-per-bar"/> chords per bar

there will be one or two chords per bar, for visibility reason.
if a bar has more than 2 chords, this will be in annotation

## <a id="line-repeat"/> line repeat

it will be possible to specify a line repeat. In this case, we will have a symbol that shows
the line has to be played this number of repeats.

the line repeat value will be either 2, 3 or 4.

the bar count numbering and time will have to be coherent with the time repeat


## <a id="text-rendering"/> text rendering

text rendering will have these features :

1. foreground color
2. background color
3. internal and external links
4. italic, bold, and other fonts
5. foot notes
