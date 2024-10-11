/*!
A framework-like crate to compose and synthetisize music. It's possible to work
without external tools just using this crate.

# General principles

Most important key information on how this crate is structured.

## Terms

A whole music piece that would usually reside in a dedicated sound file is
called a **composition**.

A composition is made of several **sections** that are put behind after one
another. A section has a defined speed (BPM) and Music Key. These parameters
can change with every new section.

A section contains several **tracks**. Unlike sections, all tracks of a section
are played at the same time. The track stores the individual notes and is
assigned an **instrument**. Tracks are also designed to be reusable.

An instrument is a user-defined implementation to generate sound given a note or
frequency. Here the sound synthesis happens. There are predefined instruments
included, but it's encouraged to implement own instruments.

## Modularity

A lot of things are implemented using exposed traits. There are default
implementations included. In some cases it's encouraged for the user to
implement their own functions (like with instruments), in other cases an
alternative implementation can be done when necessary.

### Custom note systems

It's possible to implement custom note systems. An implementation for the
standard 12-TET note system is already provided, which will serve most uses.
The most related alternative would be the 24-TET note system, which the user
has to implement themself, if they want to use it.

A common non-standard note system is e.g. actions for a drumset. Here it doesn't
make much sense to use notes, but rather actions. The user needs to implement
this system for themself. [TODO: Reference to example with this]

### Custom track handling

There are a few provided implementations for placing notes on a Track. If these
do not satisfy the needs of the user, they can implement a custom version of
the Track.

# Usage

## Placing notes on a track

It's best to always put this in a dedicated function, as we will make use of
`use` in local scope. This way tracks can also be used across different
instruments.

```rust
use synth_music::prelude::*;

fn track_example<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
where
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    use tet12::*;        // Note values
    use note::length::*; // Note lengths

    let mut track = UnboundTrack::new(instrument);
    track.set_intensity(0.7);

    // Placing notes regularly
    track.note(QUARTER, first(4));
    track.note(HALF, second(4));
    track.note(QUARTER.dot(), third(3)); // .dot() on length will extend it by half
    track.note(QUARTER, fifth(3).sharp()); // .sharp() on height will increment by one semitone

    // Placing notes quickly via macro
    sequential_notes!(track, EIGTH,
        first(4),
        seventh(3).sharp(),
        sixth(3),
        fifth(3),
        fourth(3),
        third(3),
        second(3),
        first(3)
    );

    // Placing multiple notes on top of each other
    // this is a chord
    notes!(track, HALF
        first(3),
        third(3),
        fifth(3)
    );

    // Simple logic can also be applied
    for i in 1..5 {
        track.note(QUARTER, first(i));
    }

    return track;
}
```

As you can see, there are several ways to place notes. The most versatile way
is to place them individually, but it leads to a lot of repetitions, because
"track.note(" has to be called for every note.

The macros provide a way to eliminate a lot of unnecessary repetition and save
a lot of time for writing music. It's also possible to use logic or even
branching, which is unique to this style of composing.

## Reusing specific segments

We have the ability to place notes using functions. In traditional composition,
it's only possible to repeat sections of the whole piece. The function approach
is much more versatile, as it's possible to repeat only specific tracks or even
parts of tracks. This is especially useful for simple melodies like the bass.
Below is an example to demonstrate that.

```rust
use synth_music::prelude::*;

fn complicated_track<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
where
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    use tet12::*;
    use note::length::*;

    let mut track = UnboundTrack::new(instrument);

    // Some melody here

    // Reused note segment
    apply_chord(&mut track);

    // Some melody there

    return track;    
}

fn apply_chord(track: &mut T)
where
    T: MusicTrack
{
    use tet12::*;
    use note::length::*;

    notes!(track, HALF
        first(3),
        third(3),
        fifth(3)
    );

    notes!(track, HALF
        third(3),
        fifth(3),
        first(4)
    );
}

```

It's possible to work on mutable references of Tracks instead of returning a new
one. There are probably even more ways to do other things, so get creative with
what you got.

## Dynamics

[TODO]

## Implementing instruments

[TODO]

## UnboundTrack vs. MeasureTrack

[TODO]

*/

pub mod composer;
pub mod instrument;
pub mod file_export;
pub mod prelude;
pub mod progress_bars;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
}
