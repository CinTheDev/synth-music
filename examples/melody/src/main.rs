use std::time::Duration;

use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let key = MusicKey {
        base: MusicKeyBase::A,
        key_type: MusicKeyType::Minor,
    };

    let instrument_softbass = SoftBass::new(1.0);
    let instrument_hardbass = HardBass::new(10);

    let melody_begin = track_melody_begin(Box::new(instrument_softbass));
    let chords_begin = track_chords_begin(Box::new(instrument_softbass));
    let bass_begin = track_bass_begin(Box::new(instrument_hardbass));

    let section = Section {
        info: SectionInfo {
            bpm: 120.0,
            key,
            time_signature: (4, 4),
        },

        tracks: vec![melody_begin, chords_begin, bass_begin],
    };

    let composition = Composition {
        sections: vec![section],
    };

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn track_melody_begin(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, Second, 4).dotted();
    track.note(Eigth, Third, 4);
    track.note(Quarter, Fourth, 4);
    track.note(Quarter, Fith, 4);

    track.note(Quarter, Third, 4);
    track.note(Quarter, First, 4);
    track.note(Quarter, Fith, 4);
    track.note(Eigth, Fourth, 4);
    track.note(Eigth, Third, 4);

    track.note(Quarter, Second, 4).dotted();
    track.note(Eigth, Third, 4);
    track.note(Quarter, Fourth, 4);
    track.note(Quarter, Fith, 4);

    track.note(Quarter, Third, 4);
    track.note(Quarter, First, 4);
    track.note(Half, First, 4);

    return track;
}

fn track_chords_begin(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    for _ in 0..2 {
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
    }

    return track;
}

fn track_bass_begin(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    for _ in 0..2 {
        // Chord V
        track.note(Whole, Fith, 0);

        // Chord I
        track.note(Whole, First, 1);
    }

    return track;
}

fn track_melody_repeated(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.note(Quarter, Sixth, 3).dotted();
    track.note(Eigth, Seventh, 3);
    track.note(Quarter, First, 4);
    track.note(Eigth, Seventh, 3);
    track.note(Eigth, Sixth, 3);

    track.note(Quarter, Fith, 3).dotted();
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fith, 3);
    track.note(Eigth, Fourth, 3);
    track.note(Eigth, Third, 3);

    track.note(Quarter, Second, 3);
    track.note(Eigth, Second, 3);
    track.note(Eigth, Third, 3);
    track.note(Quarter, Fourth, 3);
    track.note(Quarter, Fith, 3);

    track.note(Quarter, Third, 3);
    track.note(Quarter, First, 3);
    track.note(Half, First, 3);

    return track;
}

fn track_chords_repeated(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.set_intensity(0.2);

    // Chord IV
    for _ in 0..4 {
        track.note(Eigth, Fourth, 2);
        track.note(Eigth, First, 3);
    }

    // Chord III
    for _ in 0..4 {
        track.note(Eigth, Third, 2);
        track.note(Eigth, Seventh, 2);
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

fn track_bass_repeated(instrument: Box<dyn Instrument>) -> Track {
    use note::Tone::*;
    use note::Length::*;
    let mut track = Track::new(instrument);

    track.set_intensity(0.15);

    // Chord IV
    track.note(Whole, Fourth, 1);

    // Chord III
    track.note(Whole, Third, 1);

    // Chord V
    track.note(Whole, Fith, 0);

    // Chord I
    track.note(Whole, First, 1);

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
struct SoftBass {
    decay_speed: f32,
}

#[derive(Clone, Copy)]
struct HardBass {
    harmonics: u32,
}

impl SoftBass {
    pub fn new(decay_speed: f32) -> Self {
        Self {
            decay_speed,
        }
    }

    fn triangle_wave(info: ToneInfo) -> f32 {
        use std::f64::consts::PI;
        let x = info.time.as_secs_f64() * info.frequency * 2.0 * PI;
        x.sin().asin() as f32
    }

    fn decay_function(&self, info: ToneInfo) -> f32 {
        0.5_f32.powf(info.time.as_secs_f32() * self.decay_speed)
    }
}

impl HardBass {
    pub fn new(harmonics: u32) -> Self {
        Self {
            harmonics,
        }
    }

    fn sine_wave(time: Duration, frequency: f64) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn harmonic(n: u32, info: &ToneInfo) -> f32 {
        let factor = (2 * n + 1) as f32;
        Self::sine_wave(info.time, info.frequency * factor as f64) / factor
    }
}

impl Instrument for SoftBass {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        Self::triangle_wave(info) * self.decay_function(info)
    }
}

impl Instrument for HardBass {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        let mut amplitude = 0.0;

        for n in 0..self.harmonics {
            amplitude += Self::harmonic(n, &info);
        }

        return amplitude;
    }
}
