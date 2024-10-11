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
assigned an **instrument**.

An instrument is a user-defined implementation to generate sound given a note or
frequency. Here the sound synthesis happens. There are predefined instruments
included, but it's encouraged to implement own instruments.

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
