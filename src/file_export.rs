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

pub fn render<T: Instrument>(track: &ExportTrack<T>, settings: CompositionSettings) -> SoundBuffer {
    use indicatif::ProgressBar;

    let mut buffer = SoundBuffer::new(Vec::new(), 0, settings);

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
            settings,
            &track.instrument,
        );
        buffer.append(tone_buffer);
    }

    progress.finish_and_clear();
    
    if contains_loud_samples(&buffer) {
        progress.println("WARNING: Track contains very loud samples. Play back at your own risk.");
    }

    return buffer;
}

fn render_tone<T: Instrument>(
    tone: &Tone<T::ConcreteValue>,
    settings: CompositionSettings,
    instrument: &T
) -> SoundBuffer {
    let samples =
        (tone.play_duration.as_secs_f32() * settings.sample_rate as f32)
        .floor() as usize;

    let played_samples =
        (tone.tone_duration.as_secs_f32() * settings.sample_rate as f32)
        .floor() as usize;

    let buffer_info = BufferInfo {
        sample_rate: settings.sample_rate,
        tone_samples: played_samples,
    };

    let instrument_buffer = instrument.render_buffer(buffer_info, tone);

    let mut sound_buffer = SoundBuffer::new(
        instrument_buffer.samples,
        samples,
        settings,
    );

    apply_fade_amplitude(&mut sound_buffer);
    sound_buffer.extend_to_active_samples();

    return sound_buffer;
}

fn apply_fade_amplitude(buffer: &mut SoundBuffer) {
    let sample_rate = buffer.settings().sample_rate as f64;

    let fade_in_samples = (sample_rate * DEFAULT_FADE_IN.as_secs_f64()).ceil() as usize;
    let fade_out_samples = (sample_rate * DEFAULT_FADE_OUT.as_secs_f64()).ceil() as usize;

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

fn contains_loud_samples(buffer: &SoundBuffer) -> bool {
    for sample in &buffer.samples {
        let value = (*sample).abs();

        if value > 1.0 {
            return true;
        }
    }

    return false;
}

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! section {
    ( $section_info:expr, $( $track:expr ),+ $(,)? ) => {
        {
            use indicatif::ProgressBar;
            use synth_music::count;

            let settings = $section_info.settings.to_owned();
            
            // Progress bar
            let amount_tracks = count!($($track)*);
            
            let progress = ProgressBar::new(amount_tracks as u64)
                .with_style(synth_music::default_progress_style())
                .with_message("Section");
            
            let progress = unsafe {
                synth_music::add_progress_bar(progress)
            };
            
            // Multithreading
            use std::sync::mpsc;
            use std::thread;
            
            let (tx, rx) = mpsc::channel();
            
            $(
                let tx_thread = tx.clone();
                let export_track = $track.convert_to_export_track($section_info);
                
                thread::spawn(move || {
                    // Rendering single tracks
                    let export_buffer = file_export::render(&export_track, settings);
                    tx_thread.send(export_buffer).unwrap();
                });
            )*
            
            drop(tx);
            let mut buffer = SoundBuffer::new(Vec::new(), 0, settings);

            // Mixing all tracks together
            while let Ok(export_buffer) = rx.recv() {
                progress.inc(1);
                buffer = buffer.mix(export_buffer);
            }

            progress.finish();
            buffer
        }
    };
}

#[macro_export]
macro_rules! composition {
    ( $first_section:expr, $( $section:expr ),* $(,)? ) => {
        {
            let mut buffer = $first_section.clone();

            $(
                buffer.append($section.clone());
            )*

            buffer
        }
    };
}
