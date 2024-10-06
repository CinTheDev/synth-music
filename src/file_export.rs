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
    let mut buffer = SoundBuffer::new(sample_rate);

    instrument.generate_sound(&mut buffer, tone);

    /*
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

        let sample_value =
            instrument.generate_sound(tone, time)
            * get_fade_amplitude(tone.tone_duration, time);

        buffer.push(sample_value);
    }
    
    for _ in 0..silent_samples {
        buffer.push(0.0);
    }
    */

    return buffer;
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
