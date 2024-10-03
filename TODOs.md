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

## Improve rendering/export system

### Have instruments return a whole tone buffer

Currently the Instrument trait only provied a way to calculate individual
samples just given the tone frequency and a time. This is good enough for
instruments that use predictable sine-waves and such, but it makes other
methods that operate on the whole buffer impossible.

It should be easy enough for instruments to return the whole buffer.

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
