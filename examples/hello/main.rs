use synth_music::prelude::*;

fn main() {
    println!("Hello example");

    example_1();
    example_2();
    example_3();
}

fn example_1() {
    use tet12::*;
    use note::length::*;

    let four_four =
        TimeSignature::new(4, 4)
        .set_beat(0, 2.0)
        .set_beat(2, 1.5);

    let two_four =
        TimeSignature::new(2, 4)
        .set_beat(0, 2.0);

    let mut track1 = MeasureTrack::new(SineGenerator, four_four);
    let mut track2 = MeasureTrack::new(SineGenerator, two_four);

    track1.set_intensity(0.5);
    track2.set_intensity(0.5);

    sequential_notes!(track1, QUARTER,
        first(3),
        second(3),
        third(3),
        fourth(3)
    );
    track1.measure().unwrap();
    sequential_notes!(track1, QUARTER,
        fifth(3),
        sixth(3),
        seventh(3),
        first(4)
    );
    track1.measure().unwrap();

    sequential_notes!(track2, QUARTER,
        fifth(3),
        sixth(3)
    );
    track2.measure().unwrap();
    sequential_notes!(track2, QUARTER,
        seventh(3),
        first(4)
    );
    track2.measure().unwrap();
    sequential_notes!(track2, QUARTER,
        second(4),
        third(4)
    );
    track2.measure().unwrap();
    sequential_notes!(track2, QUARTER,
        fourth(4),
        fifth(4)
    );
    track2.measure().unwrap();

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::C,
            key_type: KeyType::Major,
        },

        settings: &settings,
    };

    let section = section!(section_info,
        track1,
        track2,
    );

    export_buffer(section, "first_example.wav");
}

fn example_2() {
    use tet12::*;
    use note::length::*;

    let instrument = predefined::SineGenerator;

    let mut track = UnboundTrack::new(instrument);

    track.note(WHOLE, third(1));

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::C,
            key_type: KeyType::Major,
        },

        settings: &settings,
    };

    let section = section!(section_info, track);

    export_buffer(section, "sound_test.wav");
}

fn example_3() {
    use tet12::*;
    use note::length::*;

    let instrument = predefined::SineGenerator;

    let four_four = TimeSignature::new(4, 4);

    let mut track = MeasureTrack::new(instrument, four_four);
    track.set_intensity(0.7);

    sequential_notes!(track, QUARTER,
        first(3),
        second(3),
        third(3),
        fourth(3)
    );
    track.measure().unwrap();

    sequential_notes!(track, QUARTER.triole(),
        first(3),
        third(3),
        second(3),

        third(3),
        fifth(3),
        fourth(3)
    );
    track.measure().unwrap();

    sequential_notes!(track, QUARTER.ntole(5),
        first(3),
        second(3),
        third(3),
        fourth(3),
        fifth(3),

        third(3),
        fourth(3),
        fifth(3),
        fourth(3),
        third(3)
    );
    track.measure().unwrap();

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::A,
            key_type: KeyType::Minor,
        },

        settings: &settings,
    };

    let section = section!(section_info, track);

    export_buffer(section, "Triole_Test.wav");
}

fn export_buffer(buffer: SoundBuffer, name: &str) {
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
