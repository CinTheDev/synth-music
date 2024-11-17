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
    window_overlap: usize,
) {
    let sample_rate = buffer.settings().sample_rate;

    let mut index_start = 0;
    let mut index_end = fft_len;

    while index_end <= buffer.samples.len() {
        let samples = &mut buffer.samples[index_start .. index_end];
        filter_fft_part(samples, sample_rate, frequency_amplitude);

        let offset = fft_len - window_overlap;
        index_start += offset;
        index_end += offset;
    }

    // TODO: Filter remaining part

    /*
    let number_of_transforms = buffer.samples.len() / fft_len;
    let sample_rate = buffer.settings().sample_rate;

    for transform_index in 0..number_of_transforms {
        let index_start = transform_index * fft_len;
        let index_end = (transform_index + 1) * fft_len;

        let samples = &mut buffer.samples[index_start .. index_end];
        filter_fft_part(samples, sample_rate, frequency_amplitude);
    }
    */

    /*
    let remaining_start_index = fft_len * number_of_transforms;

    if remaining_start_index < buffer.samples.len() {
        let remaining_samples = &mut buffer.samples[remaining_start_index..];
        filter_fft_part(remaining_samples, sample_rate, frequency_amplitude);
    }
    */
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

fn hann_window(n: usize, n_max: usize) -> f32 {
    use std::f32::consts::PI;
    let n = n as f32;
    let n_max = n_max as f32;

    return 0.5 - 0.5 * (2.0 * PI * n / n_max).cos();
}
