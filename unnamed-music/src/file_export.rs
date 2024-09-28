pub mod wav_export;

use std::time::Duration;

use crate::melody::{export_info::{ExportMusicPiece, ExportSection, Tone}, prelude::Instrument};

pub trait FileExport {
    fn export(&self, buffer: MusicBuffer) -> std::io::Result<()>;
}

pub struct MusicBuffer {
    piece: ExportMusicPiece,
}

impl MusicBuffer {
    pub fn new(piece: ExportMusicPiece) -> Self {
        Self {
            piece,
        }
    }

    pub fn generate_whole_buffer(&self, sample_rate: u32) -> Vec<f32> {
        let mut buffer: Vec<f32> = Vec::new();

        for section in &self.piece.sections {
            let mut section_buffer = Self::generate_section(section, sample_rate);
            buffer.append(&mut section_buffer);
        }

        return buffer;
    }

    fn generate_section(section: &ExportSection, sample_rate: u32) -> Vec<f32> {
        let mut buffer = Vec::new();

        // TODO: Do all tracks
        let track = &section.tracks[0];
        for tone in &track.tones {
            let mut tone_buffer = Self::generate_tone(tone, sample_rate, &track.instrument);
            buffer.append(&mut tone_buffer);
        }

        return buffer;
    }

    fn generate_tone(tone: &Tone, sample_rate: u32, instrument: &Box<dyn Instrument>) -> Vec<f32> {
        let mut buffer = Vec::new();

        let samples =
            (tone.play_duration.as_secs_f32() * sample_rate as f32)
            .floor() as u32;

        let played_samples =
            (tone.tone_duration.as_secs_f32() * sample_rate as f32)
            .floor() as u32;

        let silent_samples = samples - played_samples;

        for i in 0..played_samples {
            let time = Duration::from_secs_f64(
                i as f64 / samples as f64 * tone.play_duration.as_secs_f64()
            );

            let mut sample_value = 0.0;

            for frequency in &tone.frequencies {
                sample_value += instrument.generate_sound(*frequency as f64, time) * tone.intensity;
            }

            buffer.push(sample_value);
        }
        for _ in 0..silent_samples {
            buffer.push(0.0);
        }
        return buffer;
    }
}
