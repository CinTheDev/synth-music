# Synthetic Music Composer

A Rust library to synthesize sounds and to create music from scratch. There is
no GUI, melodies are written out programmatically. For documentation and
examples, check the
[crate documentation](https://docs.rs/synth-music/0.2.0/synth_music/). The crate
provides tools for composing music, synthetisizing sounds, and exporting the
results into a WAV file.

For getting started, check the documentation front page and the "Hello" example.

## Components of the library

The crate consists of three main parts:

- Composing music
- Defining Instruments to synthetisize sounds
- Exporting music

## Workflow

The library has been designed to be able to work without external tools. Right
now, the support for external tools is very limited, but will be improved in the
future.

The upside is that there's no required IDE, all of the project info is stored
within the source code itself. A second advantage is that this is usually more
versatile than a GUI, especially with sound synthesis.

The downside is that there's no GUI, which can e.g. display tracks in an
intuitive way. As of now, music must be composed using this library, it's
impossible to import a MIDI file from another program (Though, it's planned
to change in the future).

## TODO files

There are a few TODO files in this repository. These contain ideas for future
(or the currently being developed) versions for improvement. These can serve
as something like a roadmap or feature requests, but there's no 100% gurantee
that these will be implemented (e.g. if a better alternative has popped up).

Once these TODOs or tasks are finished, they will be deleted.

I'm glad to accept help if you want to contribute to the project.
