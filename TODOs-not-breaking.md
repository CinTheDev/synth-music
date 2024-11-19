# Additions / fixes that won't introduce breaking changes

A collection of ideas and improvements that won't introduce breaking changes.
An update with only unbreaking changes will increment the third number of the
version.

After completion of a specific task here the description will be deleted.
Though, through versioning (git) it's possible to review finished (and therefore
deleted) tasks. Other ideas for future versions shall be written in another
document.

## Split track feature

In traditional music notation (or at least in MuseScore) it's possible to
"split" a staff into two, where both parts will no play independently. This
is common for percussion, but not so much for melodies.

This could be implemented as a track function that if called, consumes the
current track and returns two more. Later, these should be merged together
again automatically.

## More control on attack, decay, sustain, etc

The library has a hard-coded attack and decay to prevent unpleasant artifacts
because of extreme onsets or tone cutoffs. Though, with specific cases this
might interfere with desired sound. Furthermore, if users want to extend the
attack duration, it should be easily done. Same goes for decay, sustain, and
other values.

## Feature held notes (for MeasureTrack)

A thing you can do in music theory is to "combine" two consecutive notes with
the same height so that they sound like one. The special thing is that these
connected notes can go beyond measures, which would be impossible with
MeasureTrack right now.

I feel like it would be pretty hard to do the same for sections, so just aim
for the MeasureTrack version first.

## Construct Track with MIDI file

If the original way of composing Tracks placing individual notes is undesired,
it should be possible to read a MIDI file from the disk and automatically
convert it to notes on a Track.

Check the contents of a MIDI file and see how this could be implemented.

## More synthesizer tools

Provide a bunch of default implementations for things often needed for
synthesizing.

### More easy usage for FFT filtering

The current filter requires a closure that will compute the amplitude of the
given frequency. On one hand, this allows the user to have maxmimum control
over the filter, but on the other hand, it's tedious to use it this way,
especially if smoother functions are desired.

There are already a few wrapper functions for hard frequency cutoffs, but such
hard cutoffs don't sound great.

The problem with providing such wrappers is that they severly limit the possible
amount of curves representable.

Please think about this problem, it's probably best to come up with some
"function builder" struct that will make creating a smooth curve very easy.

Also consider transformations (e.g. linear), because falloffs might need to
operate on the logarithmic frequency instead of linear.

## More exporter tools

A collection of ideas for generic exporter features
