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
    let fft_len = buffer.samples.len();

    let mut planner = RealFftPlanner::new();
    let fft_forward = planner.plan_fft_forward(fft_len);
    let fft_inverse = planner.plan_fft_inverse(fft_len);

    let mut spectrum = fft_forward.make_output_vec();

    fft_forward.process(&mut buffer.samples, &mut spectrum).unwrap();

    let delta = buffer.settings().sample_rate as f32 / 2.0 / fft_len as f32;

    for i in 0..spectrum.len() {
        let frequency = i as f32 * delta;
        let factor = frequency_amplitude(frequency);
        spectrum[i] *= factor;
    }

    fft_inverse.process(&mut spectrum, &mut buffer.samples).unwrap();

    for sample in buffer.samples.iter_mut() {
        *sample /= fft_len as f32;
    }
}

pub fn filter_fft_sized(
    buffer: &mut SoundBuffer,
    frequency_amplitude: fn(f32) -> f32,
    fft_len: usize,
) {
    let mut planner = RealFftPlanner::new();
    let fft_forward = planner.plan_fft_forward(fft_len);
    let fft_inverse = planner.plan_fft_inverse(fft_len);

    let mut spectrum = fft_forward.make_output_vec();

    // TODO: Also transform the "end bit" that does not fit a whole transform
    let number_of_transforms = buffer.samples.len() / fft_len;

    let delta = buffer.settings().sample_rate as f32 / 2.0 / fft_len as f32;

    for transform_index in 0..number_of_transforms {
        let index_start = transform_index * fft_len;
        let index_end = (transform_index + 1) * fft_len;

        let samples = &mut buffer.samples[index_start .. index_end];
        fft_forward.process(samples, &mut spectrum).unwrap();

        for i in 0..fft_len/2 {
            let frequency = i as f32 * delta;
            let factor = frequency_amplitude(frequency);
            spectrum[i] *= factor;
        }

        fft_inverse.process(&mut spectrum, samples).unwrap();

        for sample in samples.iter_mut() {
            *sample /= fft_len as f32;
        }
    }

    let valid_length = number_of_transforms * fft_len;
    
    for i in valid_length..buffer.samples.len() {
        buffer.samples[i] = 0.0;
    }
}
