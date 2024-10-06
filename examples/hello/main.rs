use synth_music::prelude::*;

fn main() {
    println!("Hello example");

    example_1();
}

fn example_1() {
    use tet12::*;
    use note::Length::*;

    let mut track1 = MeasureTrack::new(SineGenerator, (4, 4));
    let mut track2 = MeasureTrack::new(SineGenerator, (2, 4));

    sequential_notes!(track1, Quarter,
        first(3),
        second(3),
        third(3),
        fourth(3)
    );
    track1.measure().unwrap();
    sequential_notes!(track1, Quarter,
        fifth(3),
        sixth(3),
        seventh(3),
        first(4)
    );
    track1.measure().unwrap();

    sequential_notes!(track2, Quarter,
        fifth(3),
        sixth(3)
    );
    track2.measure().unwrap();
    sequential_notes!(track2, Quarter,
        seventh(3),
        first(4)
    );
    track2.measure().unwrap();
    sequential_notes!(track2, Quarter,
        second(4),
        third(4)
    );
    track2.measure().unwrap();
    sequential_notes!(track2, Quarter,
        fourth(4),
        fifth(4)
    );
    track2.measure().unwrap();

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::C,
            key_type: KeyType::Major,
        },
        time_signature: (4, 4),
    };

    let section = section!(section_info, 44100,
        track1,
        track2
    );

    export_buffer(section, "first_example.wav");
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
    pub fn generate(info: &Tone<tet12::TET12ConcreteTone>, time: Duration) -> f32 {
        let mut result = 0.0;

        for tone in &info.concrete_values {
            let frequency = tone.to_frequency() as f64;
            result += Self::generate_frequency(frequency, time);
        }

        return result * info.intensity.start;
    }

    pub fn generate_frequency(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl Instrument for SineGenerator {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn generate_sound(&self, buffer: &mut SoundBuffer, info: &Tone<Self::ConcreteValue>) {
        for i in 0..buffer.samples.len() {
            let time = buffer.get_time_from_index(i);
            buffer.samples[i] = Self::generate(info, time);
        }
    }
}
