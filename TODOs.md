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

### Better handling of technical values

The main culprit here is that especially the sample rate is required basically
everywhere in the rendering process, and the user has to pass it into pretty
much every function. If we add more features later, it will become really
inconvinient.

Think of a way to efficiently handle such settings, while still keeping them
seperate from exporter settings (E.g. WavExporter).

### Multithreading

As long as the architecture allows it, try to include multithreading when
rendering multiple instruments at once. Since there is no way to hold multiple
Tracks in the same struct, the only way (I see right now) to abstract this away
from the user is to use macros, or include it in the already provided macros.

Though, this feature is optional. Do not implement it if things get too messy.

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

### Unit tests

It would be good to at least implement the easy unit tests.
