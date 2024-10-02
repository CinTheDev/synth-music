# Notes on Section Erase

While trying to make the program take generic note systems, the architecture
started to not work out anymore. It just doesn't provide the flexibility that
I want. This problem occurs with Sections specifically.

I will rework the architecture, which will probably involve getting rid of
the Section (and Composition) struct.

## Problem in more detail

As I already said, the issue has to do with Sections. Sections have to do one
very special thing: They need to store multiple Tracks. A Track struct is a
generic because it can have an arbitrary Instrument and the Note System of that
Instrument. (Tracks before exporting also have the Scaled version of the note,
so these are actually double-generic!)

For this to make sense, Sections need to store Tracks independent of the generic
type. If we made Sections generic as well, then we could e.g. only use one
specific Instrument for every Instrument, which is wrong. It needs to be
possible to use different Instruments and different Note systems.

I initially solved this problem by making Instruments an enum, and having the
user define this enum with their instruments. But since I did the basically
same thing with the note system, it doesn't work out anymore. It also makes
no sense to do the same enum trick with the note-system, because every
instrument is assigned a specific note-system, that cannot be represented as an
arbitrary (generic) type.

`dyn Trait` also doesn't work because the Instrument Trait requires a type to be
specified, and the dyn also requires it to be specified, which would again force
every track to use the same type.

I also tried to "hide" the generics behind a wrapper trait, which initially did
work, but later made other crucial functionality impossible like returning the
generic type.

This problem essentially boils down to storing multiple generic structs
independent of the generic type, without using `dyn`, wrapper Traits, or
enums. I don't think this is possible with my current code architecture.

## General idea

Since I believe the problem I'm trying to solve is impossible (because of bad
architecture), I'm changing the architecture to completely avoid this.

Basically, it is now forbidden to store multiple Tracks inside a struct like
Sections do. What Sections did was to pass multiple Tracks into a render
function which then gave us a raw buffer of audio sample to store as a file.

### Idea 1: Using macros for exporting

Instead storing Tracks inside Sections, it would be better to pass Tracks
directly from User side into the render functions. To make this pleasant to the
user, we'll use macros that look similar to Sections (only that they immediatly
render out the Tracks.)
