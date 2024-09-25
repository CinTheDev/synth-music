# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

## Support multiple tracks

Right now only one track and one instrument can be used. Improve implementation
so that any number of tracks is possible.

## Remove instrument encapsulation

Right now, tracks are wrapped inside instruments. Right now, instruments do not
contain any info about sound, so there's no additional info there.

This has the side effect that instruments have to be recreated for every section
in a composition, which feels unnecessary. Especially later when instruments will
contain more info about themselves, this will add a lot of repeated code.

Rework the structure so that a section contains Tracks, and every track must be
assigned an instrument.

## Missing features for composing

There are some missing features that are crucial for composing music.

### Octaves

Make it possible for notes to reach other octaves

### Dotted notes and trioles

Note lengths cannot be modified as of now, change that so dotted notes and maybe
even trioles are possible.

### Sharps and flats

It should be possible to manually sharpen of flatten a note. Optionally several
times.

### Music Key

The library right now just ignores the specified music key. Make it possible
to use different keys, including major and minor keys.

### Intervals and pauses

For now, the function to place a note only takes a single value. This is by
design to be as short as possible. Though, tones are actually stored as vectors
to be able to represent intervals or even pauses.

Convert the note placement function into a macro so the amount of parameters
is variable. With this it should be possible to provide multiple tones or
even no tones to represent a pause.

### Seperation of measures

Measures are kind of invisible/non-existant in code right now. This isn't
the end of the world, the code works perfectly fine without it. Though
when it comes to workflow, it's a bit unusual to have no measures.

Think about a workflow feature to visibly seperate notes out into measures.

### Crescendo and decrescendo

A (de)crescendo represents a transition of the intensity. Since a (de)crescendo
should be able to change the intensity while a note plays, it needs to be
represented individually.

Implement it and restructure if needed.

## Sound samples

TODOs regarding generation of sound samples

### Base of Sound samples

Create the base of the submodule and integrate it into the library.

