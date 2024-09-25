pub mod instrument;
use instrument::Instrument;

pub enum MusicKey {
    C,
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

                for tone in note.values {
                    frequencies.push(get_note_frequency(tone));
                }

                let play_duration = note.length.get_duration(section.bpm);

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

fn get_note_frequency(tone: instrument::note::Tone) -> f32 {
    use instrument::note::Tone;
    match tone {
        Tone::First => get_frequency_from_a4(-9),
        Tone::Second => get_frequency_from_a4(-7),
        Tone::Third => get_frequency_from_a4(-5),
        Tone::Fourth => get_frequency_from_a4(-4),
        Tone::Fith => get_frequency_from_a4(-2),
        Tone::Sixth => get_frequency_from_a4(0),
        Tone::Seventh => get_frequency_from_a4(2),
    }
}

fn get_frequency_from_a4(semitones: i32) -> f32 {
    2_f32.powf(semitones as f32 / 12.0) * 440.0
}
