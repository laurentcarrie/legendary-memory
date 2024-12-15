# bf files
use [birdfont](https://birdfont.org/)  on linux to edit the *.bf files. Then use the export menu to generate
the *.ttf files. The ttf files are versioned for convenience, but are a result of the bf files.

# to use the fonts in latex :

    cp *.ttf $HOME/.fonts

# to use the fonts in word processors :

    mkdir -p $HOME/.local/share/fonts
    cp *.ttf $HOME/.local/share/fonts/.
    fc-cache -f -v

# 3 files
- lolo.bf has the chords A..G, the 7, m...
- lolo_flat has the same thing, for the flat chords
- lolo_sharp has the same thing, for the sharp chords

# chords.latex
update the macros in [chords.tex](../others/texfiles/chords.tex)
