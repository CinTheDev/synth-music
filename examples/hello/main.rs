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

    //export_track(track1.convert_to_export_track(section_info), "first_example.wav");
    export_buffer(section, "first_example.wav");
}

use export_info::ExportTrack;
fn export_track<T: Instrument>(track: ExportTrack<T>, name: &str) {
    let buffer = file_export::render(&track, 44100);
    export_buffer(buffer, name);
}

fn export_buffer(buffer: Vec<f32>, name: &str) {
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
    pub fn generate(frequency: f64, time: Duration) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }
}

impl Instrument for SineGenerator {
    type ConcreteValue = tet12::TET12ConcreteTone;

    fn generate_sound(&self, info: ToneInfo<Self::ConcreteValue>) -> f32 {
        let frequency = info.tone.to_frequency() as f64;
        Self::generate(frequency, info.time)
    }
}
