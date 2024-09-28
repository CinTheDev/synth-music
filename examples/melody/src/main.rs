use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let first_key = MusicKey {
        base: MusicKeyBase::C,
        key_type: MusicKeyType::Major,
    };
    let second_key = MusicKey {
        base: MusicKeyBase::A,
        key_type: MusicKeyType::Minor,
    };

    let sine_generator = Box::new(SineGenerator);
    let harmonic_generator = Box::new(HarmonicSineGenerator::new(100));

    let section_1 = Section {
        bpm: 120.0,
        key: first_key,
        time_signature: (4, 4),

        tracks: vec![track_1(sine_generator)],
    };
    let section_2 = Section {
        bpm: 120.0,
        key: second_key,
        time_signature: (4, 4),

        tracks: vec![track_2(harmonic_generator)],
    };

    let composition = Composition {
        sections: vec![section_1, section_2],
    };

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn track_1(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, First, 4);
    track.note(Quarter, Second, 4);
    track.note(Quarter, Third, 4);
    track.note(Quarter, Fourth, 4);
    track.note(Quarter, Fith, 4).staccato();
    track.note(Quarter, Sixth, 4);
    track.note(Quarter, Seventh, 4);
    track.note(Quarter, First, 5);

    return track;
}

fn track_2(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, First, 4).dotted();
    track.note(Eigth, Second, 4);
    track.note(Quarter, Third, 4).dotted();
    track.note(Eigth, First, 4);
    track.note(Eigth, Third, 4);
    track.note(Eigth, Third, 4);
    track.note(Eigth, Second, 4);
    track.note(Eigth, First, 4);
    track.note(Quarter, Second, 4);
    track.note(Quarter, Fith, 3);

    return track;
}

fn export(export_piece: ExportMusicPiece) {
    use unnamed_music::file_export::*;
    use wav_export::WavExport;
    use std::path::PathBuf;

    let music_buffer = MusicBuffer::new(export_piece);
    let exporter = WavExport {
        path: PathBuf::from("export/debug.wav"),
        sample_rate: 44100,
        ..Default::default()
    };

    exporter.export(music_buffer).unwrap();
}

#[derive(Clone, Copy)]
struct SineGenerator;

impl Instrument for SineGenerator {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        use std::f64::consts::PI;
        (info.time.as_secs_f64() * info.frequency * 2.0 * PI).sin() as f32
    }
}

#[derive(Clone, Copy)]
struct HarmonicSineGenerator {
    count: u32,
}

impl HarmonicSineGenerator {
    pub fn new(count: u32) -> Self {
        Self {
            count,
        }
    }

    fn get_harmonic_frequency(n: u32, info: &ToneInfo) -> f32 {
        use std::f64::consts::PI;
        (info.time.as_secs_f64() * info.frequency * 2.0 * PI * n as f64).sin() as f32
    }

    fn dropoff(n: u32) -> f32 {
        0.5_f32.powi(n as i32)
    }
}

impl Instrument for HarmonicSineGenerator {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        let mut value = 0.0;

        for i in 0..self.count {
            value += Self::get_harmonic_frequency(i, &info) * Self::dropoff(i);
        }

        return value;
    }
}
