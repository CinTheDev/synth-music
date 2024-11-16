/*!
A framework-like crate to compose and synthetisize music. It's possible to work
without external tools just using this crate. There are provided tools for
composing music, synthetisizing sounds, and exporting the results into a file.

# General principles

Most important key information on how this crate is structured.

## Terminology

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
this system for themself. Check the "custom_note_system" example for details.

### Custom track handling

There are a few provided implementations for placing notes on a Track. If these
do not satisfy the needs of the user, they can implement a custom version of
the Track.

### Custom exporter

There's a provided implementation for WAV files. The user can implement
exporting to other file formats such as mp3, ogg, etc...

# Usage

## Placing notes on a track

It's best to always put this in a dedicated function, as we will make use of
`use` in local scope. This way tracks can also be used across different
instruments.

```rust
use synth_music::prelude::*;

fn main() {

    let instrument = predefined::SineGenerator;

    // Somewhere in main

    let track = track_example(instrument);
}

fn track_example<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
where
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    use tet12::*;        // Note values
    use length::*; // Note lengths

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
    notes!(track, HALF,
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

use tet12::*;
use length::*;

let mut track = UnboundTrack::new(predefined::SineGenerator);

// Some melody here

// Reused note segment
apply_chord(&mut track);

// Some melody there

// ...

fn apply_chord<T, U>(track: &mut T)
where
    T: MusicTrack<TET12ScaledTone, U>,
    U: Instrument<ConcreteValue = TET12ConcreteTone>,
{
    use tet12::*;
    use length::*;

    notes!(track, HALF,
        first(3),
        third(3),
        fifth(3)
    );

    notes!(track, HALF,
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
use tet12::*;
use length::*;

let mut track = UnboundTrack::new(predefined::SineGenerator);

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

All trait functions of `Instrument` already have a default implementation. This
is to avoid repeating the same implementation, since synthesis for almost all
instruments works the same. If you do not implement any functions, the
instrument will only render silence.

Before going to an example, a brief explanation on all trait functions sorted
after how likely you're going to need to override them:

- `render_sample` - Render a sample of a tone at a given time. Use this if the
samples can be computed independent of each other (given the time). The
amplitude of the tone should always be at `1.0`.

- `get_intensity` - Return the intensity at a given time. Override if you want
the intensity e.g. to become quieter with time.

- `get_num_samples` - Return the amount of samples the buffer should consist of
in total.

- `post_process` - Called after everything else with write access to the buffer.

- `render_tone_buffer` - Wraps `render_sample`, override if you need to render
the whole buffer for a single tone in a single function. `render_sample` will
become useless (will never be called) if you override this and not call it
yourself.

- `apply_intensity` - Wraps `get_intensity` the same way as `render_tone_buffer`
does with `render_sample`.

- `render` - This function is called by the crate during the rendering stage,
and it calls all other functions above for rendering. Only overwrite if you want
to have absolute control over the render. If you do override this function,
every other function here will become useless (not called) if you do not do so
yourself.

Look for the `Instrument` documentation for more details

The buffer works with f32 samples, where 1.0 or -1.0 are the maximum amplitude,
which should generally not be exceeded. The output buffer can be shorter or
longer than expected, it will then automatically get extended or mixed with the
following tones.

Now, on to the example:

```rust
use synth_music::prelude::*;
use tet12::TET12ConcreteTone;
use std::time::Duration;

#[derive(Clone, Copy)]
struct ExampleInstrument {
    count: u32,
    decay_speed: f32,
}

impl ExampleInstrument {
    pub fn new(count: u32, decay_speed: f32) -> Self {
        Self {
            count,
            decay_speed,
        }
    }

    // Wave function (sine)
    fn wave(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    // Exponential decay over time
    fn decay_factor(&self, time: Duration) -> f32 {
        0.5_f32.powf(time.as_secs_f32() * self.decay_speed)
    }
}

impl Instrument for ExampleInstrument {
    // Specify that this instrument operates on 12-TET notes
    type ConcreteValue = TET12ConcreteTone;

    // The sample consists of several harmonic frequencies.
    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        let mut sample = 0.0;

        for n in 0..self.count {
            let factor = (2 * n + 1) as f64;
            sample += Self::wave(frequency * factor, time);
        }

        return sample;
    }

    // The intensity will become exponentially quieter with time
    fn get_intensity(&self, tones: &Tone<Self::ConcreteValue>, time: Duration) -> f32 {
        let base = tones.intensity.start;
        let factor = self.decay_factor(time);

        return base * factor;
    }
}
```

This is a simple instrument that has a variable amount of harmonics in its base
sine-wave tone. These harmonics don't get quieter with higher frequencies, so
the sound is harsh and loud, especially with many harmonics.

This is the most simple use case, which you'll need 90% of the time when the
instrument is based on predictable waves. If you needed access to the whole
buffer while rendering for some reason, you would need to implement
`render_tone_buffer`, etc..

For more examples please look into the examples folder.

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

```no_run
use synth_music::prelude::*;

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
let instrument = predefined::SineGenerator;

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
#
# fn track_begin_melody<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
# where
#     T: Instrument<ConcreteValue = TET12ConcreteTone>
# {
#     return UnboundTrack::new(instrument);    
# }
#
# fn track_begin_bass<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
# where
#     T: Instrument<ConcreteValue = TET12ConcreteTone>
# {
#     return UnboundTrack::new(instrument);    
# }
#
# fn track_end_melody<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
# where
#     T: Instrument<ConcreteValue = TET12ConcreteTone>
# {
#     return UnboundTrack::new(instrument);    
# }
#
# fn track_end_bass<T>(instrument: T) -> UnboundTrack<TET12ScaledTone, T>
# where
#     T: Instrument<ConcreteValue = TET12ConcreteTone>
# {
#     return UnboundTrack::new(instrument);    
# }
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
is [here](`composer::TimeSignature`).

```rust
use synth_music::prelude::*;
use tet12::*;
use length::*;

let instrument = predefined::SineGenerator;
// 4/4 Time
let time_signature = TimeSignature::new(4, 4);

let mut track = MeasureTrack::new(instrument, time_signature);
track.set_intensity(0.7);

// After four Quarters the measure must end
sequential_notes!(track, QUARTER,
    first(3),
    second(3),
    third(3),
    fourth(3)
);
track.measure().unwrap();
sequential_notes!(track, QUARTER,
    fifth(3),
    sixth(3),
    seventh(3),
    first(4)
);
// The last measure must also be "placed" with this call.
track.measure().unwrap();
```

Now an example that is wrong:

```should_panic
use synth_music::prelude::*;
use tet12::*;
use length::*;

let instrument = predefined::SineGenerator;
let time_signature = TimeSignature::new(4, 4);

let mut track = MeasureTrack::new(instrument, time_signature);
track.set_intensity(0.7);

sequential_notes!(track, QUARTER,
    first(3),
    second(3),
    third(3),
    fourth(3)
);
track.measure().unwrap();
sequential_notes!(track, QUARTER,
    fifth(3),
    sixth(3),
    seventh(3)
    // missing a note; there should be a break here
);
track.measure().unwrap(); // panics here

sequential_notes!(track, QUARTER,
    fifth(3),
    sixth(3),
    seventh(3),
    first(4),
    second(4) // one note too much, the measure should've ended earlier
);
track.measure().unwrap(); // panics here

sequential_notes!(track, QUARTER,
    fifth(3),
    sixth(3),
    seventh(3),
    first(4)
);
// not calling track.measure() will not place these notes in the track.
```

### UnboundTrack

UnboundTrack is like MeasureTrack without Measures. One can arbitrarily place
notes, the "position" of the notes is not enforced to be anywhere. The user can
also resort to this implementation if the manual placing of Measures is too
tedious.

This implementation is best used for small scale tests and not complicated
melodies. It's easy to accidentally break the regularity of the music, and there
are no measure boundaries that serve as orientation points for the user.

```rust
use synth_music::prelude::*;
use tet12::*;
use length::*;

let mut track = UnboundTrack::new(predefined::SineGenerator);
track.set_intensity(0.7);

// There is no limit to placing notes
sequential_notes!(track, EIGTH.dot(),
    first(3),
    second(3),
    third(3),
    fourth(3),
    fifth(3),
    sixth(3),
    seventh(3)
);

sequential_notes!(track, QUARTER.triole(),
    first(4),
    third(4),
    first(4)
);
```

### Custom implementation

The user can also provide their own implementation for Tracks. For more info,
check [`composer::MusicTrack`].

*/

pub mod composer;
pub mod instrument;
pub mod file_export;
pub mod prelude;
#[doc(hidden)]
pub mod progress_bars;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
}
