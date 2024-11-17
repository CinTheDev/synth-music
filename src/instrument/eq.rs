use super::SoundBuffer;
use realfft::RealFftPlanner;

/// Apply a FFT filter across the whole buffer, where the FFT length matches
/// the buffer length. Only do this for periodic signals (e.g. tones).
/// 
/// This function will decompose the signal into its frequencies using a FFT,
/// and will multiply them with numbers given by `frequency_amplitude`. This
/// argument is a function that is given a frequency, and must return the
/// desired amplitude for that frequency. At the end the signal is reconstructed
/// using the altered frequencies.
pub fn filter_fft_whole(buffer: &mut SoundBuffer, frequency_amplitude: fn(f32) -> f32) {
    let sample_rate = buffer.settings().sample_rate;
    filter_fft_part(&mut buffer.samples, sample_rate, frequency_amplitude);
}

pub fn filter_fft_sized(
    buffer: &mut SoundBuffer,
    frequency_amplitude: fn(f32) -> f32,
    fft_len: usize,
) {
    let number_of_transforms = buffer.samples.len() / fft_len;
    let sample_rate = buffer.settings().sample_rate;

    // First pass: Filter using FFT
    for transform_index in 0..number_of_transforms {
        let index_start = transform_index * fft_len;
        let index_end = (transform_index + 1) * fft_len;

        let samples = &mut buffer.samples[index_start .. index_end];
        filter_fft_part(samples, sample_rate, frequency_amplitude);
    }

    let remaining_start_index = fft_len * number_of_transforms;

    if remaining_start_index < buffer.samples.len() {
        let remaining_samples = &mut buffer.samples[remaining_start_index..];
        filter_fft_part(remaining_samples, sample_rate, frequency_amplitude);
    }

    // Second pass: Make all parts seamless
    for transform_index in 0..(number_of_transforms-1) {
        let first_index_start = transform_index * fft_len;

        let (_, first_buffer) = buffer.samples.split_at_mut(first_index_start);
        let (first_buffer, second_buffer) = first_buffer.split_at_mut(fft_len);
        let (second_buffer, _) = second_buffer.split_at_mut(fft_len);

        make_seamless(first_buffer, second_buffer, 100);
    }
}

fn filter_fft_part(
    buffer: &mut [f32],
    sample_rate: u32,
    frequency_amplitude: fn(f32) -> f32,
) {
    let fft_len = buffer.len();

    let mut planner = RealFftPlanner::new();
    let fft_forward = planner.plan_fft_forward(fft_len);
    let fft_inverse = planner.plan_fft_inverse(fft_len);

    let mut spectrum = fft_forward.make_output_vec();

    fft_forward.process(buffer, &mut spectrum).unwrap();

    let delta = sample_rate as f32 / 2.0 / fft_len as f32;

    for i in 0..spectrum.len() {
        let frequency = i as f32 * delta;
        let factor = frequency_amplitude(frequency);
        spectrum[i] *= factor;
    }

    fft_inverse.process(&mut spectrum, buffer).unwrap();

    for sample in buffer.iter_mut() {
        *sample /= fft_len as f32;
    }
}

pub fn make_seamless(first: &mut [f32], second: &mut [f32], distance: usize) {
    let end_first = first.last().unwrap();
    let start_second = second.first().unwrap();

    let midpoint = (end_first + start_second) / 2.0;

    for i in 0..distance {
        let index_end = first.len() - 1 - i;
        let index_start = i;
        let t = 1.0 - (i as f32 / distance as f32);

        lerp(first[index_end], midpoint, t);
        lerp(second[index_start], midpoint, t);
    }
}

// TODO: Move this to a more general place
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    t * (b - a) + a
}
