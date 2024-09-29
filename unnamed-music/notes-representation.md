# Notes Representation

This is a temporary document for helping with the Notes redesign.

## General requirements

A loose list of requirements that the design should handle easily.

### A note can have n number of values

This also includes 0 values. 0 values represent a pause, while multiple values
are a chord or similar.

### Every single value can be sharpened or flattened individually

The emphasis here is that **every single value** can be modified in height.
The old system has the problem that in a chord individual values cannot be
sharpened or flattened.

That means the representation of sharps or flats should be in the value itself,
not in the whole note structure (the note structure holds all notes that play
together).

### The general note length can be modified

This is similar to above with note values, but the note length must be the same
for all values in a chord. The old system did that correctly, so keep it the
same way.

### Notes are stored independent of key

A note value should be representable independently of the section key. This
makes handling notes later on during export or render much easier.

Though, when placing notes during the composition process, the key does matter.
More info on that later.

### Placed notes are abstract values

When placing a note, we don't want to think about the specific value that it is,
because it will change when the key is changed. With the old system these
abstract values are called "First", "Second", up until "Seventh". The "First"
value would be a C if the key is C Major for example.

This system has a flaw though, it contradicts the requirement that individual
values can be modified in height, e.g. sharpened or flattened, unless we
introduce additional values like "FirstSharp". Though, that's kinda messy and
we should look for a better approach.

Independent of that, when notes are placed the composer should directly convert
these to concrete values like "C#" or "A".

### It should be possible to use other (user defined) note systems

This is the biggest requirement yet. Right now, only the standard 12-TET notes
can be used, and because they're directly converted to raw frequencies, they're
not that flexible.

The major issue with this is that if the user wants to use an instrument that
does not operate on notes (e.g. a drumset), it makes no real sense to represent
notes like that.

To fix that, there need to be two major changes:

#### Make notes represented by a trait or similar

So that different note systems can be used.

#### Pass note values to instrument function

Currently the instrument sound function receives a frequency to generate a
sound. With the changes above it would make more sense to pass the concrete
note values to the instrument function.

The conversion from 12-TET note values to frequencies could be handled by an
external function that can be called from the instrument sound function.

### More advanced dynamics representation

The intensity of a note is currently represented only as a single float between
0.0 and 1.0.

Often, there's a term like "acceleration" of tones, which relates to intensity.
The acceleration of a tone describes how "hard" the tone begins to sound.
This behaviour is individual to the instrument, so that must be handled by the
user. *The user shall get the needed information to do such calculations.*

Also, later on we want to implement (de)crescendos, which are basically a smooth
transition of the intensity. Though, in one case there's multiple ways how this
could be handled.

Imagine a long held note, and a crescendo is applied over the duration of the
note. That means that at the beginnning of the note the intensity is low, and
at the end of it the intensity is high.

Now, if the instrument operates on "pulses" (e.g. piano, guitar), the intensity
is determined at the beginning of the note but remains unchanged for the
duration of the note, the crescendo is effectively ignored. Often the notes of
these instruments become quiter when held for longer time.

But if the instrument operates on "lengths" (e.g. strings, brass, woodwinds),
the intensity can change during the note duration. Here the intensity would
slowly increase because of the crescendo. Often (without crescendo) the notes
of these instruments would have constant loudness when held for longer time.

Since both behaviours are possible, even for two instruments playing at the
same time, the type of behaviour should be (easily) determined by the user.

## Representation of ExportTone

The export tones are generated from composition tones, and are given to the
instrument sound generator to render a tone.

An ExportTone represents the following info:

- All concrete Tones
- A *total* duration (time until the next tone)
- A *tone* duration (time for which tone is played)
- An intensity or transition of intensity

Note: the "fade-in" or "fade-out" times should still be applied in rendering,
but it does not need to be stored for every tone individually.
