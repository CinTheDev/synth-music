# Additions / fixes that won't introduce breaking changes

A collection of ideas and improvements that won't introduce breaking changes.
An update with only unbreaking changes will increment the third number of the
version.

After completion of a specific task here the description will be deleted.
Though, through versioning (git) it's possible to review finished (and therefore
deleted) tasks. Other ideas for future versions shall be written in another
document.

## Add function in MeasureTrack to get mutable reference to active Measure

Since we want to be able to modify a measure *before* validating it, we'll need
a function to get the active measure. Returning a reference to a validated
measure is not only pointless, but might actually corrupt the measure (if
valid data is overwritten with invalid data).

## Get rid of redundancy between MeasureTrack and UnboundTrack

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

## Shortcut traits for instruments

Users have to implement basically everything for every instrument over and over
again, which gets really repetitive. This is only because we cannot generalize
instruments to e.g. have predictable wave functions, and therefore not need to
know about other values in the same buffer. These "shortcut traits" should make
the struct also implement Instrument, and handle some of these general
implementations. If possible, make it possible that multiple shortcut traits
can be used at once.

Following features could be implemented as shortcut traits:

- Instead of all tones at once only call for one tone
- Instead of filling whole buffer call for every sample
- Automatic dynamics calculations

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

### Noise generation

Provide functions for generating multiple types of noise. White noise is the
easiest because it's just random values, but using the EQ above we can create
other types by filtering white noise.

### Wave functions with phase

Make it possible to specify the phase (which is unaffected by frequency) for
the provided wave functions. If necessary, provide two seperate functions for
every wave type where one has specifiable phase, whereas the other behaves the
same as right now (phase is always zero).

### Better buffer handling

Right now, handling buffers inside an `Instrument` impl is cumbersome, because
the buffers are raw `Vec<f32>`s, and other important info like `buffer_info`
have to be handled seperately. Furthermore, some useful buffer implementations
like mixing two variable length buffers should also be provided, so the user
doesn't have to implement that for themself.
