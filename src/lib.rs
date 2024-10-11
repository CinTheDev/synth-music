/*!
A framework-like crate to compose and synthetisize music. It's possible to work
without external tools just using this crate. There are provided tools for
composing music, synthetisizing sounds, and exporting the results into a file.

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

Dynamics in music refer to the loudness or intensity at which should be played.
These dynamics can change dynamically (no pun intended) throughout the song.

```rust
use synth_music::prelude::*;

fn example_dynamics<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
where
    T: Instrument<ConcreteValue = TET12ConcreteValue>
{
    use tet12::*;
    use note::length::*;

    let mut track = UnboundTrack::new(instrument);

    // Set the intensity for new notes to be 70% of maximum volume
    track.set_intensity(0.7);

    track.note(HALF, first(3)); // Intensity = 0.7
    track.note(HALF, first(3)); // Intensity = 0.7

    track.set_intensity(0.2);

    track.note(HALF, first(3)); // Intensity = 0.2


    // Track will start changing intensity arriving at value specified later
    track.start_dynamic_change();
    track.note(QUARTER, third(3));
    track.note(QUARTER, fourth(3));
    track.note(QUARTER, fifth(3));
    track.note(QUARTER, first(3));
    // Marks the end of the dynamic change, the passed value is the target intensity.
    // This target intensity is now the actual intensity of the track.
    track.end_dynamic_change(0.6);

    return track;    
}
```

Calling `track.set_intensity(x)` will change the intensity of the notes placed
afterwards. This is equivalent to a dynamics marker in traditional music
notation (e.g. "f" for "forte" or "loud"; "p" for "piano" or "quiet").

The "dynamic change" will smoothly transition the intensity from the currently
set intensity of the track to the value specified at "end_dynamic_change". In
traditional music this is called "crescendo" for becoming louder or
"decrescendo" for becoming quieter. This crate does not differentiate becoming
louder or becoming quieter.

## Implementing instruments

Instruments represent the entire sound synthesis part of this crate. Here, most
implementation is left to the user, the crate will only provide useful info for
generating the sound, but the actual sound synthesis is the responsibility of
the user. There is an exposed trait `Instrument` that needs to be implemented.

Before going to an example, a brief explanation on what the Trait expects
from the user:

- The Trait provides info regarding the output buffer and the input tones.
- The output buffer should generally have `buffer_info.tone_samples` entries.
- A `tone` actually can represent multiple tones at once (like in a chord), every value needs to be handled.
- There are provided functions for calculating a time from the sample index
- The whole output buffer represents a whole note / multiple stacked notes.
- Dynamics are handled by the user

The buffer works with f32 samples, where 1.0 or -1.0 are the maximum amplitude,
which should generally not be exceeded. The output buffer can be shorter or
longer than expected, it will then automatically get extended or mixed with the
following tones.

Now, on to the example:

```rust
use synth_music::prelude::*;
use tet12::TET12ConcreteTone;

struct ExampleInstrument {
    count: u32,
}

impl ExampleInstrument {
    pub fn new(count: u32) -> Self {
        Self {
            count,
        }
    }

    // Generate the tones given a point in time
    fn generate_tones(&self, tones: &Tone<TET12ConcreteTone>, time: Duration) -> f32 {
        let mut sample_amplitude = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            sample_amplitude += self.generate_frequency(frequency, time);
        }

        return sample_amplitude * tones.intensity.start;
    }

    // Generate a single tone given a point in time and frequency
    // The tone contains multiple harmonics
    fn generate_frequency(&self, frequency: f64, time: Duration) -> f32 {
        let mut sample_amplitude = 0.0;

        for n in 0..self.count {
            let factor = (2 * n + 1) as f32;
            sample_amplitude += self.wave(frequency * factor, time);
        }

        return sample_amplitude;
    }

    // Wave function (sine)
    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl Instrument for ExampleInstrument {
    // Specify that this instrument operates on 12-TET notes
    type ConcreteValue = TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        // We will push the required amount of samples to completely fill the length of the tone
        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(self.generate(tones, time));
        }

        InstrumentBuffer { samples: buffer }
    }
}
```

This is a simple instrument that has a variable amount of harmonics in it's base
sine-wave tone. These harmonics don't get quieter with higher frequencies, so
the sound is harsh and loud, especially with many harmonics.

The instrument struct implements all of the sound generation, while the code
directly within the trait implementation only manages the buffer, and calls
the sound generation functions. Though, you can implement this however you want
or need to. Stuff like post-processing the tone should also be possible within
the trait implementation.

This example does not implement features like making the tone quieter with time
(similar to a piano), and adjusting the intensity throughout the tone.

The reason why the user needs to implement that is because if we have an
instrument like a piano during a crescendo, one long note wouldn't get louder,
while several short notes would get progressively louder. An instrument like
a trumpet can have constant loudness over a note, or even dynamically change
the intensity throughout a note. Neither of these things are implemented in
this example.

For examples that do implement these things, check the examples folder of this
crate.

## Exporting

Exporting is the last step of making music. In this stage all the instrument
implementations will actually be called and the tracks will be rendered and
mixed into sections, which will then be combined into the final composition and
written to a file.

There are two types of information that will be important to define here. There
are the **settings** which apply for the whole composition, and **SectionInfo**,
which apply only to one section. The settings include technical values like
sample rate, and SectionInfo is info that can change during a composition, like
the speed in BPM or the music key.

Unfortunately, due to the Tracks being highly generic Traits, it's currently not
possible to store several tracks together (e.g. in a Vec) without forcing their
types to be the absolute same. To combat this, sections are actually just
represented by a macro call and directly rendered into a Buffer. Another macro
call can then piece these buffers together into a composition.

```rust
use synth_music::prelude::*;

fn render() {

    // Define settings
    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    // Info for beginning. Info always contains a reference to the settings.
    let info_begin = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::A,
            key_type: KeyType::Minor,
        },

        settings: &settings,
    };

    let info_end = SectionInfo {
        bpm: 140.0,
        key: MusicKey {
            tonic: KeyTonic::Asharp,
            key_type: KeyType::Minor,
        },

        settings: &settings,
    };

    // Any instruments work, different ones can be used for different tracks
    let instrument = // [an instrument]

    let track_begin_melody = track_begin_melody(instrument);
    let track_begin_bass = track_begin_bass(instrument);

    let track_end_melody = track_end_melody(instrument);
    let track_end_bass = track_end_bass(instrument);

    // Render the first section with the specified tracks and info.
    // The result will already be a rendered buffer
    let section_begin = section!(info_begin,
        track_begin_melody,
        track_begin_bass,
    );

    let section_end = section!(info_end,
        track_end_melody,
        track_end_bass,
    );

    // The rendered sections are put together to create the whole music piece.
    let composition = composition!(
        section_begin,
        section_end,
    );

    export(composition, "my_beautiful_piece.wav");
}

fn export(buffer: SoundBuffer, name: &str) {
    use std::path::PathBuf;

    // Create struct with relevant info about exporting
    // This will create a file on the file system.
    let exporter = WavExport {
        path: PathBuf::from(name),
        ..Default::default()
    };
    exporter.export(buffer).unwrap();
}
```

## UnboundTrack vs. MeasureTrack

As of now there are two implementations for MusicTrack to place notes on a
track.

Before anything, it's recommended to use **MeasureTrack** for regular use,
because it enforces more rules from music theory. Most examples use UnboundTrack
for more compact code and concentration on the presented point.

### MeasureTrack

MeasureTrack is used for placing notes *and* measure bounds. A measure must
always be filled with the right amount of notes and breaks to be "saturated".
Trying to work with unsaturated threads will likely result in a panic.

This enforcing of measure bounds serves to prevent mistakes where the measure
bounds are violated and the track becomes desynchronized with the rest.

MeasureTrack also provides access to the time signature features. More on this
is [TODO: reference location]

[TODO: Example]

### UnboundTrack

UnboundTrack is like MeasureTrack without Measures. One can arbitrarily place
notes, the "position" of the notes is not enforced to be anywhere. The user can
also resort to this implementation if the manual placing of Measures is too
tedious.

This implementation is best used for small scale tests and not complicated
melodies. It's easy to accidentally break the regularity of the music, and there
are no measure boundaries that serve as orientation points for the user.

[TODO: Example]

### Custom implementation

The user can also provide their own implementation for Tracks. More on this
[TODO: reference location]

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
