pub mod export_info;
pub mod wav_export;

use std::time::Duration;

use export_info::*;
use crate::instrument::{Instrument, BufferInfo};

const DEFAULT_FADE_IN: Duration = Duration::from_millis(2);
const DEFAULT_FADE_OUT: Duration = Duration::from_millis(2);

pub trait FileExport {
    fn export(&self, buffer: SoundBuffer) -> std::io::Result<()>;
}

pub fn render<T: Instrument>(track: &ExportTrack<T>, sample_rate: u32) -> SoundBuffer {
    let mut buffer = SoundBuffer::new(Vec::new(), sample_rate, 0);

    for tone in &track.tones {
        let tone_buffer = render_tone(
            tone,
            sample_rate,
            &track.instrument,
        );
        buffer.append(tone_buffer);
    }

    return buffer;
}

fn render_tone<T: Instrument>(tone: &Tone<T::ConcreteValue>, sample_rate: u32, instrument: &T) -> SoundBuffer {
    let samples =
        (tone.play_duration.as_secs_f32() * sample_rate as f32)
        .floor() as usize;

    let played_samples =
        (tone.tone_duration.as_secs_f32() * sample_rate as f32)
        .floor() as usize;

    let buffer_info = BufferInfo {
        sample_rate,
        tone_samples: played_samples,
    };

    let instrument_buffer = instrument.render_buffer(buffer_info, tone);

    let mut sound_buffer = SoundBuffer::new(
        instrument_buffer.samples,
        sample_rate,
        samples,
    );

    apply_fade_amplitude(&mut sound_buffer, tone.tone_duration);
    sound_buffer.extend_to_active_samples();

    return sound_buffer;
}

fn apply_fade_amplitude(buffer: &mut SoundBuffer, tone_duration: Duration) {
    for i in 0..buffer.samples.len() {
        let time = buffer.time_from_index(i);
        buffer.samples[i] *= get_fade_amplitude(tone_duration, time);
    }
}

fn get_fade_amplitude(tone_duration: Duration, time: Duration) -> f32 {
    if DEFAULT_FADE_IN > tone_duration || DEFAULT_FADE_OUT > tone_duration {
        return 1.0;
    }

    // Apply fade-in
    if time < DEFAULT_FADE_IN {
        let t = time.as_secs_f32() / DEFAULT_FADE_IN.as_secs_f32();
        return fade_in_smooth(t);
    }
    // Apply fade-out
    else if time > tone_duration - DEFAULT_FADE_OUT {
        let t_time = time - (tone_duration - DEFAULT_FADE_OUT);
        let t = t_time.as_secs_f32() / DEFAULT_FADE_OUT.as_secs_f32();
        return fade_out_smooth(t);
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
    fade_in_smooth(1.0 - t)
}

#[macro_export]
macro_rules! section {
    ( $section_info:expr, $sample_rate:expr, $( $track:expr ),+ ) => {
        {
            let mut buffer = SoundBuffer::new(Vec::new(), $sample_rate, 0);

            $(
                let export_track = $track.convert_to_export_track($section_info);
                let export_buffer = file_export::render(&export_track, $sample_rate);
                buffer = buffer.mix(export_buffer);
            )*

            buffer
        }
    };
}
