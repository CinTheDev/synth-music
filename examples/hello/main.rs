use synth_music::prelude::*;

fn main() {
    println!("Hello example");

    // Create implemented instrument and tracks
    let example_instrument = SineInstrument;
    let track = create_track(example_instrument);

    // Specify settings (must be constant throughout the whole composition)
    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    // Specify attributes (can change if section transition occurs)
    let info = SectionInfo {
        bpm: 120.0,
        key: music_key::C_MAJOR,
        settings: &settings,
    };

    // Render the section, it's possible to enter any number of tracks.
    // All tracks will play at the same time, parallel to one another.
    let section = section!(info,
        track,
    );

    // Append all sections together, any number of sections is accepted.
    // In this example the given section is repeated.
    let composition = composition!(
        section,
        section
    );

    // Export the composition. Check the function implementation below.
    export(composition);
}

fn create_track<T>(instrument: T) -> MeasureTrack<TET12ScaledTone, T>
where
    T: Instrument<ConcreteValue = TET12ConcreteTone>
{
    // For reducing excessive repetition
    use tet12::*;
    use length::*;

    // Define a time signature
    let four_four = TimeSignature::new(4, 4);

    // Create a new track
    let mut track = MeasureTrack::new(instrument, four_four);

    // Place a bunch of notes behind each other with constant length
    sequential_notes!(track, QUARTER,
        first(3),
        second(3),
        third(3),
        fourth(3),
    );
    // Place the end of the measure
    track.measure().unwrap();

    sequential_notes!(track, QUARTER,
        fifth(3),
        sixth(3),
        seventh(3),
        first(4),
    );
    track.measure().unwrap();

    track.pause(WHOLE);
    track.measure().unwrap();

    // Change the intensity throughout the melody
    track.set_intensity(0.2);

    // You can also place notes without a macro
    track.note(QUARTER, first(3));
    track.note(QUARTER, third(3));
    track.note(QUARTER, fifth(3));
    track.note(QUARTER, third(3));
    track.measure().unwrap();
    track.note(WHOLE, first(3));
    track.measure().unwrap();

    // Notes placed this way can be modified afterwards
    track.note(QUARTER, first(4)).staccato();
    track.note(QUARTER, third(4)).staccato();
    track.note(QUARTER, fifth(4)).staccato();
    track.note(QUARTER, third(4));
    track.measure().unwrap();
    
    // This will stack notes, so these will be played at the same time
    notes!(track, WHOLE,
        first(4),
        fifth(3),
        third(3),
    );
    track.measure().unwrap();

    // Of course, note stacks can be modified too
    notes!(track, QUARTER,
        first(4),
        fifth(3),
        third(3),
    ).staccato();
    notes!(track, QUARTER,
        first(4),
        fifth(3),
        third(3),
    ).staccato();
    notes!(track, HALF,
        first(4),
        fifth(3),
        third(3).flat(), // Flatten the third to create a minor chord
    );
    track.measure().unwrap();

    return track;
}

fn export(buffer: SoundBuffer) {
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    // Specify new exporter with given attributes
    let wav_export = WavExport {
        path: PathBuf::from("export/Hello.wav"),
        ..Default::default()
    };

    // Actually export the piece.
    wav_export.export(buffer).unwrap();
}

// Instruments

use std::time::Duration;

// Create a new instrument that will output notes as sine waves.
#[derive(Clone, Copy)]
struct SineInstrument;

impl SineInstrument {
    // Outputs the amplitude of the corresponding sine-wave at the specified time
    pub fn generate_frequency(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

// Required to implement to use as an Instrument
impl Instrument for SineInstrument {
    type ConcreteValue = TET12ConcreteTone;

    fn render_sample(&self, tone: Self::ConcreteValue, time: Duration) -> f32 {
        let frequency = tone.to_frequency() as f64;
        return Self::generate_frequency(frequency, time);
    }
}
