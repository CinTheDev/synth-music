use std::time::Duration;

use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let key = MusicKey {
        base: MusicKeyBase::A,
        key_type: MusicKeyType::Minor,
    };

    let instrument_lead = Lead::new();
    let instrument_softbass = SoftBass::new();
    let instrument_hardbass = HardBass::new(10);

    let melody = track_melody(Box::new(instrument_lead));
    let chords = track_chords(Box::new(instrument_softbass));
    let bass = track_bass(Box::new(instrument_hardbass));

    let section = Section {
        info: SectionInfo {
            bpm: 120.0,
            key,
            time_signature: (4, 4),
        },

        tracks: vec![melody, chords, bass],
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

fn track_bass(instrument: Box<dyn Instrument>) -> Track {
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

struct Lead {

}

struct SoftBass {

}

struct HardBass {
    harmonics: u32,
}

impl Lead {
    pub fn new() -> Self {
        Self { }
    }

    pub fn sine_wave(info: ToneInfo) -> f32 {
        use std::f64::consts::PI;
        (info.time.as_secs_f64() * info.frequency * 2.0 * PI).sin() as f32
    }
}

impl SoftBass {
    pub fn new() -> Self {
        Self { }
    }

    fn triangle_wave(info: ToneInfo) -> f32 {
        use std::f64::consts::PI;
        let x = info.time.as_secs_f64() * info.frequency * 2.0 * PI;
        x.sin().asin() as f32
    }
}

impl HardBass {
    pub fn new(harmonics: u32) -> Self {
        Self {
            harmonics,
        }
    }

    //fn square_wave(info: ToneInfo) -> f32 {
    //    let value = (info.time.as_secs_f64() * info.frequency).floor() as u32;
    //    if value % 2 == 1 {
    //        return 1.0;
    //    }
    //    else {
    //        return -1.0;
    //    }
    //}

    fn sine_wave(time: Duration, frequency: f64) -> f32 {
        use std::f64::consts::PI;
        (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32
    }

    fn harmonic(n: u32, info: &ToneInfo) -> f32 {
        let factor = (2 * n + 1) as f32;
        Self::sine_wave(info.time, info.frequency * factor as f64) / factor
    }
}

impl Instrument for Lead {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        Self::sine_wave(info)
    }
}

impl Instrument for SoftBass {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        Self::triangle_wave(info)
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
