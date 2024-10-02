fn main() {
    println!("Hello example");

    example_1();
}

fn example_1() {
    use unnamed_music::composer::Track;
    use unnamed_music::instrument::predefined::tet12::*;
    use unnamed_music::composer::note::Length::*;
    use unnamed_music::composer::SectionInfo;
    use unnamed_music::composer::music_key::*;

    let mut track = Track::new(SineGenerator);

    unnamed_music::sequential_notes!(track, Quarter,
        first(3),
        second(3),
        third(3),
        fourth(3),
        fifth(3),
        sixth(3),
        seventh(3),
        first(4)
    );

    let section_info = SectionInfo {
        bpm: 120.0,
        key: MusicKey {
            tonic: KeyTonic::C,
            key_type: KeyType::Major,
        },
        time_signature: (4, 4),
    };

    track.convert_to_export_track(section_info);
}

// Instruments

use unnamed_music::instrument::{Instrument, ToneInfo};
use unnamed_music::instrument::predefined::tet12;

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
