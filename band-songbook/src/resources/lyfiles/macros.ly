songtempo = {{{song.info.tempo}}}

% The corpus-wide library (articulation marks, songbookBeatMarks) lives in a
% real versioned file, songs/songbook.ily, mirrored into the sandbox next to
% settings.yml - so this relative path resolves from every song directory.
% Pulling it in here means a song gets the whole library just by being built,
% with nothing to add to its own .ly files.
\include "../../songbook.ily"
