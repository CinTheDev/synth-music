use synth_music::prelude::*;

fn main() {
    println!("Hello example");
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
    track.set_intensity(0.5);

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
        third(3),
    );
    track.measure().unwrap();

    return track;
}

fn export(buffer: SoundBuffer, name: &str) {
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let wav_export = WavExport {
        path: PathBuf::from("export").join(name),
        ..Default::default()
    };

    wav_export.export(buffer).unwrap();
}

// Instruments

use std::time::Duration;

#[derive(Clone, Copy)]
struct SineGenerator;

impl SineGenerator {
    pub fn generate(tones: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &tones.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::generate_frequency(frequency, time);
        }

        return result * tones.intensity.start * tones.beat_emphasis.unwrap_or(0.5);
    }

    pub fn generate_frequency(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl Instrument for SineGenerator {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let mut buffer = Vec::new();

        for i in 0..buffer_info.tone_samples {
            let time = buffer_info.time_from_index(i);
            buffer.push(Self::generate(tones, time));
        }

        return InstrumentBuffer { samples: buffer }
    }
}
