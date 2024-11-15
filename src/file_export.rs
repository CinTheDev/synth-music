pub mod export_info;
pub mod wav_export;

use std::time::Duration;

#[doc(inline)]
pub use export_info::*;
use crate::instrument::Instrument;
use crate::progress_bars;

const DEFAULT_FADE_IN: Duration = Duration::from_millis(2);
const DEFAULT_FADE_OUT: Duration = Duration::from_millis(2);

/// Represents saving a buffer as a file on the file system.
pub trait FileExport {
    fn export(&self, buffer: SoundBuffer) -> std::io::Result<()>;
}

/// Renders an `ExportTrack` into a `SoundBuffer`
/// 
/// This function will automatically print a progress bar with the render
/// progress.
pub fn render<T: Instrument>(track: &ExportTrack<T>, settings: CompositionSettings) -> SoundBuffer {
    use indicatif::ProgressBar;

    let mut buffer = SoundBuffer::new(Vec::new(), 0, settings);

    let progress = ProgressBar::new(track.tones.len() as u64)
        .with_style(progress_bars::default_progress_style())
        .with_message("Track");

    let progress = unsafe {
        progress_bars::add_progress_bar(progress)
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
    
    if let Some(index) = contains_loud_samples(&buffer) {
        let msg = format!(
            "WARNING: Track contains very loud samples starting at sample {}; \
            t = {:?}. Play back at your own risk.",
            index,
            buffer.time_from_index(index),
        );
        progress.println(msg);
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

    let mut instrument_buffer = SoundBuffer::new(Vec::new(), played_samples, settings);
    instrument.render(tone, &mut instrument_buffer);

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

fn contains_loud_samples(buffer: &SoundBuffer) -> Option<usize> {
    for i in 0..buffer.samples.len() {
        let sample = buffer.samples[i];
        let value = sample.abs();

        if value > 1.0 {
            return Some(i);
        }
    }

    return None;
}

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

pub extern crate indicatif;

/// Render a number of tracks into a buffer with the given section info. This
/// represents a section, where all tracks play at once.
/// 
/// This will print a progress bar showing how many tracks have been rendered
/// already.
/// 
/// Internally, multithreading is used for rendering multiple tracks in
/// parallel.
#[macro_export]
macro_rules! section {
    ( $section_info:expr, $( $track:expr ),+ $(,)? ) => {
        {
            use synth_music::file_export::indicatif::ProgressBar;
            use synth_music::count;
            use synth_music::progress_bars;

            let settings = $section_info.settings.to_owned();
            
            // Progress bar
            let amount_tracks = count!($($track)*);
            
            let progress = ProgressBar::new(amount_tracks as u64)
                .with_style(progress_bars::default_progress_style())
                .with_message("Section");
            
            let progress = unsafe {
                progress_bars::add_progress_bar(progress)
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

/// Append multiple sections together to form a single buffer for the whole
/// composition. This final buffer can then be exported into a file.
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
