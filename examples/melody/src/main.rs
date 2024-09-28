use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let key = MusicKey {
        base: MusicKeyBase::A,
        key_type: MusicKeyType::Minor,
    };

    let harmonic_generator = Box::new(HarmonicSineGenerator::new(10));

    let melody = track_melody(harmonic_generator.clone());
    let chords = track_chords(harmonic_generator);

    let section = Section {
        bpm: 120.0,
        key,
        time_signature: (4, 4),

        tracks: vec![melody, chords],
    };

    let composition = Composition {
        sections: vec![section],
    };

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn track_melody(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, Sixth, 4).dotted();
    track.note(Eigth, Seventh, 4);
    track.note(Quarter, First, 5);
    track.note(Eigth, Seventh, 4);
    track.note(Eigth, Sixth, 4);

    track.note(Quarter, Fith, 4).dotted();
    track.note(Eigth, Third, 4);
    track.note(Quarter, Fith, 4);
    track.note(Eigth, Fourth, 4);
    track.note(Eigth, Third, 4);

    track.note(Quarter, Second, 4);
    track.note(Eigth, Second, 4);
    track.note(Eigth, Third, 4);
    track.note(Quarter, Fourth, 4);
    track.note(Quarter, Fith, 4);

    track.note(Quarter, Third, 4);
    track.note(Quarter, First, 4);
    track.note(Half, First, 4);

    return track;
}

fn track_chords(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    // Chord IV
    for _ in 0..4 {
        track.note(Eigth, Fourth, 2);
        track.note(Eigth, First, 3);
    }

    // Chord I
    for _ in 0..4 {
        track.note(Eigth, First, 2);
        track.note(Eigth, Fith, 2);
    }

    // Chord V
    for _ in 0..4 {
        track.note(Eigth, Fith, 1);
        track.note(Eigth, Second, 2);
    }

    // Chord I
    for _ in 0..4 {
        track.note(Eigth, First, 2);
        track.note(Eigth, Fith, 2);
    }

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
