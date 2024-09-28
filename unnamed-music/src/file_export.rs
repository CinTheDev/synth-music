pub mod wav_export;

use std::time::Duration;

use crate::melody;
use melody::export_info::{ExportMusicPiece, ExportSection, Tone};
use melody::instrument::Instrument;

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
            sample_value *= Self::get_fade_amplitude(&tone, time);

            buffer.push(sample_value);
        }
        for _ in 0..silent_samples {
            buffer.push(0.0);
        }
        return buffer;
    }

    fn get_fade_amplitude(tone: &Tone, time: Duration) -> f32 {
        // Apply fade-in
        if time < tone.fade_in {
            let t = time.as_secs_f32() / tone.fade_in.as_secs_f32();
            return Self::fade_in_smooth(t);
        }
        // Apply fade-out
        else if time > tone.play_duration - tone.fade_out {
            let t_time = time - (tone.play_duration - tone.fade_out);
            let t = t_time.as_secs_f32() / tone.fade_out.as_secs_f32();
            return Self::fade_out_smooth(t);
        }
        // Not amplitude change
        else {
            return 1.0;
        }
    }

    fn fade_in_smooth(t: f32) -> f32 {
        3.0*t*t - 2.0*t*t*t
    }
    fn fade_out_smooth(t: f32) -> f32 {
        Self::fade_in_smooth(1.0 - t)
    }

    fn fade_in_circular(t: f32) -> f32 {
        Self::fade_out_circular(1.0 - t)
    }
    fn fade_out_circular(t: f32) -> f32 {
        (1.0 - t*t).sqrt()
    }
}
