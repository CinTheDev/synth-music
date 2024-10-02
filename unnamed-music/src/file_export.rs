pub mod export_info;
/*
pub mod wav_export;

use export_info::*;
use crate::instrument::{Instrument, ToneInfo};

use std::time::Duration;

const DEFAULT_FADE_IN: Duration = Duration::from_millis(2);
const DEFAULT_FADE_OUT: Duration = Duration::from_millis(2);

pub trait FileExport<T: Instrument> {
    fn export(&self, buffer: MusicBuffer<T>) -> std::io::Result<()>;
}

pub struct MusicBuffer<T: Instrument> {
    piece: ExportMusicPiece<T>,
}

impl<T: Instrument> MusicBuffer<T> {
    pub fn new(piece: ExportMusicPiece<T>) -> Self {
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

    fn generate_section(section: &ExportSection<T>, sample_rate: u32) -> Vec<f32> {
        let mut buffer = Vec::new();

        for track in &section.tracks {
            let track_buffer = Self::generate_track(track, sample_rate);
            buffer = Self::mix_buffers(buffer, track_buffer);
        }

        return buffer;
    }

    fn generate_track(track: &ExportTrack<T>, sample_rate: u32) -> Vec<f32> {
        let mut buffer = Vec::new();

        for tone in &track.tones {
            let mut tone_buffer = Self::generate_tone(
                tone,
                sample_rate,
                &track.instrument
            );
            buffer.append(&mut tone_buffer);
        }

        return buffer;
    }

    fn generate_tone(tone: &Tone<T::ConcreteValue>, sample_rate: u32, instrument: &T) -> Vec<f32> {
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

            for value in &tone.concrete_values {
                let info = ToneInfo {
                    tone: *value,
                    time,
                    intensity: tone.intensity,
                };
                sample_value += instrument.generate_sound(info);
            }
            sample_value *= Self::get_fade_amplitude(&tone, time);

            buffer.push(sample_value);
        }
        for _ in 0..silent_samples {
            buffer.push(0.0);
        }
        return buffer;
    }

    fn get_fade_amplitude(tone: &Tone<T::ConcreteValue>, time: Duration) -> f32 {
        if DEFAULT_FADE_IN > tone.tone_duration || DEFAULT_FADE_OUT > tone.tone_duration {
            return 1.0;
        }

        // Apply fade-in
        if time < DEFAULT_FADE_IN {
            let t = time.as_secs_f32() / DEFAULT_FADE_IN.as_secs_f32();
            return Self::fade_in_smooth(t);
        }
        // Apply fade-out
        else if time > tone.tone_duration - DEFAULT_FADE_OUT {
            let t_time = time - (tone.tone_duration - DEFAULT_FADE_OUT);
            let t = t_time.as_secs_f32() / DEFAULT_FADE_OUT.as_secs_f32();
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

    fn mix_buffers(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
        let (mut larger_buffer, smaller_buffer) = match a.len() >= b.len() {
            true => (a, b),
            false => (b, a),
        };

        for i in 0..smaller_buffer.len() {
            larger_buffer[i] += smaller_buffer[i];
        }

        return larger_buffer;
    }
}
*/
