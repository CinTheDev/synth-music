pub mod export_info;
pub mod wav_export;

use std::time::Duration;

use export_info::*;
use crate::instrument::Instrument;

const DEFAULT_FADE_IN: Duration = Duration::from_millis(2);
const DEFAULT_FADE_OUT: Duration = Duration::from_millis(2);

pub trait FileExport {
    fn export(&self, buffer: SoundBuffer) -> std::io::Result<()>;
}

pub fn render<T: Instrument>(track: &ExportTrack<T>, sample_rate: u32) -> SoundBuffer {
    let mut buffer = SoundBuffer::new(sample_rate);

    for tone in &track.tones {
        let mut tone_buffer = render_tone(
            tone,
            sample_rate,
            &track.instrument,
        );
        buffer.append(&mut tone_buffer);
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

    let silent_samples = samples - played_samples;

    let mut buffer = SoundBuffer::new(sample_rate);
    buffer.preallocate(played_samples);

    instrument.generate_sound(&mut buffer, tone);

    apply_fade_amplitude(&mut buffer, tone.tone_duration);
    buffer.extend(silent_samples);

    return buffer;
}

fn apply_fade_amplitude(buffer: &mut SoundBuffer, tone_duration: Duration) {
    for i in 0..buffer.samples.len() {
        let time = buffer.get_time_from_index(i);
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
            let mut buffer = SoundBuffer::new($sample_rate);

            $(
                let export_track = $track.convert_to_export_track($section_info);
                let export_buffer = file_export::render(&export_track, $sample_rate);
                buffer = buffer.mix(export_buffer);
            )*

            buffer
        }
    };
}
