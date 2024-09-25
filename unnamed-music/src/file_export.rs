pub mod wav_export;

use std::time::Duration;

use super::melody::export_info::ExportMusicPiece;

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
        let mut time: Duration = Duration::ZERO;

        // TODO: Do all tracks
        let track = &self.piece.tracks[0];
        for tone in &track.tones {
            let samples =
                (tone.play_duration.as_secs_f32() * sample_rate as f32)
                .floor() as u32;

            let played_samples =
                (tone.tone_duration.as_secs_f32() * sample_rate as f32)
                .floor() as u32;

            let silent_samples = samples - played_samples;

            let delta_time = Duration::from_secs_f64(1.0 / sample_rate as f64);
            for _ in 0..played_samples {
                let mut sample_value = 0.0;

                for frequency in &tone.frequencies {
                    sample_value += dbg_sound_generator(*frequency as f64, time) * tone.intensity;
                }

                time += delta_time;
                buffer.push(sample_value);
            }
            for _ in 0..silent_samples {
                buffer.push(0.0);
            }
        }

        return buffer;
    }
}

fn dbg_sound_generator(frequency: f64, time: Duration) -> f32 {
    use std::f64::consts::PI;

    return (time.as_secs_f64() * frequency * 2.0 * PI).sin() as f32;
}
