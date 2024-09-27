pub mod instrument;
use instrument::Instrument;

#[derive(Clone, Copy)]
pub enum MusicKey {
    C,
    Csharp,
    Cflat,
    D,
    Dflat,
    E,
    Eflat,
    F,
    Fsharp,
    G,
    Gflat,
    A,
    Aflat,
    B,
    Bflat,
}
#[derive(Clone, Copy)]
pub enum MusicKeyType {
    Major,
    Minor,
}

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
pub struct Composition {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),

    pub instruments: Vec<Instrument>,
}

impl Composition {
    pub fn to_export_piece(self) -> crate::melody::export_info::ExportMusicPiece {
        use crate::melody::export_info::*;
        let mut result = ExportMusicPiece::new();

        // TODO: Multiple tracks
        result.tracks.push(ExportTrack::new());

        for mut section in self.sections {
            let mut instrument = section.instruments.pop().unwrap();
            let track = instrument.tracks.pop().unwrap();

            for note in track.get_notes() {
                let mut frequencies = Vec::new();

                for tone in &note.values {
                    frequencies.push(
                        modify_frequency(
                            get_note_frequency(*tone),
                            note.semitones_offset,
                        )
                    );
                }

                let play_duration = note.get_duration(section.bpm);

                result.tracks[0].tones.push(Tone {
                    frequencies,
                    play_duration,
                    tone_duration: play_duration.mul_f32(note.play_fraction),
                    intensity: note.intensity,
                })
            }
        }

        return result;
    }
}

fn get_note_frequency(tone: (instrument::note::Tone, i32)) -> f32 {
    use instrument::note::Tone;
    let base_frequency = match tone.0 {
        Tone::First => get_frequency_from_a4(-9),
        Tone::Second => get_frequency_from_a4(-7),
        Tone::Third => get_frequency_from_a4(-5),
        Tone::Fourth => get_frequency_from_a4(-4),
        Tone::Fith => get_frequency_from_a4(-2),
        Tone::Sixth => get_frequency_from_a4(0),
        Tone::Seventh => get_frequency_from_a4(2),
    };

    base_frequency.powi(tone.1 - 4)
}

fn get_frequency_from_a4(semitones: i32) -> f32 {
    2_f32.powf(semitones as f32 / 12.0) * 440.0
}

fn modify_frequency(frequency: f32, semitones: i32) -> f32 {
    2_f32.powf(semitones as f32 / 12.0) * frequency
}
