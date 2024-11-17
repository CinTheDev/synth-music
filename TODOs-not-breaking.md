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

### EQ filter using FFT

The FFT (fast fourier transform) can decompose a tone into it's frequencies and
turn those back into a tone. The cool thing is that it's possible to modify
these frequencies before turning them back, which effectively filters or even
boosts those frequencies.

TODO: Implement `custom_noise` using EQ.

## More exporter tools

A collection of ideas for generic exporter features
