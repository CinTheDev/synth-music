pub mod wav_export;

use super::melody::MusicPiece;

pub trait FileExport {
    fn export(&self, buffer: MusicBuffer) -> std::io::Result<()>;
}

pub struct MusicBuffer {
    piece: MusicPiece,
}

impl MusicBuffer {
    pub fn new(piece: MusicPiece) -> Self {
        Self {
            piece,
        }
    }

    /*
    pub fn generate_buffer(&self, start_time: Duration, end_time: Duration) -> Result<&[u8], &'static str> {
        Ok(&self.buffer)
    }
    */
    pub fn generate_whole_buffer(&self, sample_rate: u32) -> Vec<f32> {
        let mut buffer: Vec<f32> = Vec::new();

        for section in &self.piece.sections {
            // Let's do one instrument and one track for now
            // TODO: Multiple instruments and tracks
            let instrument = &section.instruments[0];
            let track = &instrument.tracks[0];

            let notes = track.get_notes();

            for note in notes {
                let quarters_per_second = section.bpm as f32 / 60.0;
                let note_time = note_length_multiplier(note.length) / quarters_per_second;
                let samples = (note_time * sample_rate as f32).floor() as u32;

                for s in 0..samples {
                    let time = s as f32 * note_time / samples as f32;

                    // TODO: custom sound generation
                    let mut sample_value = 0.0;

                    for frequency in &note.tones {
                        sample_value += dbg_sound_generator(*frequency, time) * note.intensity;
                    }

                    buffer.push(sample_value);
                }
            }
        }

        return buffer;
    }
}

fn dbg_sound_generator(frequency: f32, time: f32) -> f32 {
    use std::f32::consts::PI;

    return (time * frequency * 2.0 * PI).sin();
}
