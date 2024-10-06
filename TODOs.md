# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

**NOTE**: This document shall now only contain points that are included in the
first release version (0.1.0), the document shall be deleted once the code is
ready for release (when all points are completed). Other ideas for future
versions shall be written in another document.

## Improve time signatures & note lengths

The implementation of time signatures is kind of arbitrary right now, they're
just represented as two unsigned integers.

First step is to wrap this into a struct with more checking and functions so
it makes sense in the context of music theory.

And since time signatures are closely related to note lengths, the note lengths
should be reworked as well. Right now they're just an enum, with external flags
that control whether these are dotted or trioles. (This is suboptimal.)

Note on trioles: They are kind of special actually, and I've been ignoring these
for a bit already because they're handled incorrectly in code. The new
implementation should take regard to trioles (in a correct way of course). If we
feel like it, we can extend these to n-toles like pentoles, septoles, etc.

## Improve rendering/export system

### Provide better API for rendering / handling buffers

The current interface for rendering and handling buffers is a bit chaotic. Look
at all parts of the rendering process and try to come up with improvements to
make it more versatile, intuitive, and less sensitive to bugs (caused from bad
user input).

#### Seperate struct for Instrument SoundBuffer and user buffer

The SoundBuffer provided for instruments to fill and the finished buffer
returned to the user are the same struct, which limits a bunch of potential
functionality.

Let's list what both variants of the buffer should do.

SoundBuffer for Instrument:

- Be empty when given to Instrument
- Provide for how long the instrument should play / how many samples
- Handle shorter and longer returned buffer correctly

SoundBuffer for user:

- Be easy to export

### Chunked rendering

Currently the exporter renders the whole music piece into one buffer which then
is written to the disk. The memory consumption might not be that big of a deal
(@ 44100Hz the buffer would take around 10MB for every minute of music), but
there are is another reason to make the rendering chunked.

Rendering can take a bit because it's currently single threaded. Chunked
rendering allows for Multi-Threaded rendering of the file, which will speed up
the rendering process by quite a bit. Also, the reduced memory consumption is
also not bad.

### Loading bar for exporting

Bigger projects (even the ones from the examples) take a few seconds to export.
It would be nice to have a loading bar that shows how much of the piece has
been rendered out already. It's probably related with the chunked rendering
stuff above.

## Thorough documentation

It seems like the code structure is somewhat converging to a final state, so
documenting the code is a good idea. The README.md also belongs to this. It's
better to do this early while still familiar with the code.

Also make sure to include some examples inline with documentation, research
online what best practises for this are.

### More and better examples

All examples right now only serve testing purposes and aren't that great for
showing the library's features. Write a bunch more examples with more isolated
ideas and more comments so these can be used to learn this library.
