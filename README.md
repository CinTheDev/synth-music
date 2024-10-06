# Synthetic Music Composer

A Rust library to synthesize sounds and to create music from scratch. There is
no GUI as usual, melodies are written out programmatically.

## Components of the library

The main functionalities of this library should be:

- Rendering sound into WAV files
- Synthetisizing sound (e.g. from adding different waves)
- Providing external capability to create sound samples
- Using these samples to write out melodies with multiple tracks

## TODOs for future versions

### Get rid of redundancy between MeasureTrack and UnboundTrack

I don't want to merge both structs into one and have it take a value as which
variant it should behave, I do want to keep these seperate.

The problem is that they share a lot of functionality, so the implementation
of both repeat each other a lot. It's probably difficult to extract that into
functions.

I have an idea how to circumvent this: We could make MeasureTrack the "main"
struct with all the implementation, and the UnboundTrack will just store a
MeasureTrack and automatically place measures and fill them correctly.

There's one problem though: If we want this "conversion" to happen seamlessly,
we need to implement held notes. Held notes are notes that extend their duration
beyond a single measure.

Because these are in no way implemented right now, it only makes sense to tackle
this redundancy issue if held notes are being implemented in the future.

### Construct Track with MIDI file

If the original way of composing Tracks placing individual notes is undesired,
it should be possible to read a MIDI file from the disk and automatically
convert it to notes on a Track.

Check the contents of a MIDI file and see how this could be implemented.

### Support Stereo

This will probably require some refactoring on the rendering side. Make it
possible for Instruments to output stereo sound and export it stereo. Also think
about ways to make the Instrument implementation convenient if only Mono should
be Output.

### Even more advanced dynamics system

Features for making the dynamics system even more versatile and powerful

#### Custom interpolation curves

Currently, if we slowly invrease the intensity, the change is always linear.
Make it possible for the user to specify their own interpolation curve.

Either make them choose from an enum, or let them provide their own function,
or both.

#### Dynamics effects over Track or whole Section

My idea of this is still a little vague, but I think in some other music it
sometimes feels like the intensity of some instruments "pulse" or similar with
the rythm.

Investigate online what this is called and how it is usually implemented. If
it's not very difficult, implement it here as well.
