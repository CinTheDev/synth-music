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
    use indicatif::ProgressBar;

    let mut buffer = SoundBuffer::new(Vec::new(), sample_rate, 0);

    let progress = ProgressBar::new(track.tones.len() as u64)
        .with_style(crate::default_progress_style())
        .with_message("Track");

    let progress = unsafe {
        crate::add_progress_bar(progress)
    };

    for tone in &track.tones {
        progress.inc(1);

        let tone_buffer = render_tone(
            tone,
            sample_rate,
            &track.instrument,
        );
        buffer.append(tone_buffer);
    }

    progress.finish_and_clear();

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

    apply_fade_amplitude(&mut sound_buffer);
    sound_buffer.extend_to_active_samples();

    return sound_buffer;
}

fn apply_fade_amplitude(buffer: &mut SoundBuffer) {
    let fade_in_samples = (buffer.sample_rate() as f64 * DEFAULT_FADE_IN.as_secs_f64()).ceil() as usize;
    let fade_out_samples = (buffer.sample_rate() as f64 * DEFAULT_FADE_OUT.as_secs_f64()).ceil() as usize;

    for i in 0..fade_in_samples {
        let t = i as f32 / fade_in_samples as f32;
        let index = i;
        buffer.samples[index] *= smooth(t);
    }

    for i in 0..fade_out_samples {
        let t = i as f32 / fade_in_samples as f32;
        let index = buffer.samples.len() - i - 1;
        buffer.samples[index] *= smooth(t);
    }
}

fn smooth(t: f32) -> f32 {
    3.0*t*t - 2.0*t*t*t
}

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! section {
    ( $section_info:expr, $sample_rate:expr, $( $track:expr ),+ ) => {
        {
            use indicatif::ProgressBar;
            use synth_music::count;

            let mut buffer = SoundBuffer::new(Vec::new(), $sample_rate, 0);

            let amount_tracks = count!($($track)*);

            let progress = ProgressBar::new(amount_tracks as u64)
                .with_style(synth_music::default_progress_style())
                .with_message("Section");

            let progress = unsafe {
                synth_music::add_progress_bar(progress)
            };

            $(
                progress.inc(1);
                let export_track = $track.convert_to_export_track($section_info);
                let export_buffer = file_export::render(&export_track, $sample_rate);
                buffer = buffer.mix(export_buffer);
            )*

            progress.finish();
            buffer
        }
    };
}

// TODO: Make this note take the sample rate as an argument
#[macro_export]
macro_rules! composition {
    ( $sample_rate:expr, $( $section:expr ),* ) => {
        {
            let mut buffer = SoundBuffer::new(Vec::new(), $sample_rate, 0);

            $(
                buffer.append($section.clone());
            )*

            buffer
        }
    };
}
