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
