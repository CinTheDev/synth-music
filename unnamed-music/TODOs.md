# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

## Missing features for composing

There are some missing features that are crucial for composing music.

### Seperation of measures

Measures are kind of invisible/non-existant in code right now. This isn't the
end of the world, the code works perfectly fine without it. Though when it comes
to workflow, it's a bit unusual to have no measures.

Think about a workflow feature to visibly seperate notes out into measures.

### Crescendo and decrescendo

A (de)crescendo represents a transition of the intensity. Since a (de)crescendo
should be able to change the intensity while a note plays, it needs to be
represented individually.

Implement it and restructure if needed.

## Music Key & Notes rework (CURRENTLY ACTIVE)

Right now notes, tones, and music keys have been implemented as closly to
traditional music theory as possible. The problem is that traditional music
theory is kinda inconsistent with definitions sometimes, and the code looks
really messy.

Try to make the implementations for getting the note frequencies more efficient
and readable, ideally using as few magic numbers and long match lists as
possible. Also try to relocate the code to a better place than `composer.rs`.

### Sharps and flats inside intervals

I've noticed that it's impossible to represent and interval with one note being
sharpened or flattened. The current way of applying these things affects all
notes in the interval, it's impossible to target a single note.

This is quite unfortunate, a "common" thing in music that is impossible
because of that is for example the chord V in harmonic minor scales. With the
regular (natural) minor scale this chord would be minor, but in harmonic minor
this is almost always manually changed to be major. The only way to do this is
to sharpen the "middle" note of the chord.

Rethink the note system that such things can be represented.

## Chunked rendering

Currently the exporter renders the whole music piece into one buffer which then
is written to the disk. The memory consumption might not be that big of a deal
(@ 44100Hz the buffer would take around 10MB for every minute of music), but
there are is another reason to make the rendering chunked.

Rendering can take a bit because it's currently single threaded. Chunked
rendering allows for Multi-Threaded rendering of the file, which will speed up
the rendering process by quite a bit. Also, the reduced memory consumption is
also not bad.
