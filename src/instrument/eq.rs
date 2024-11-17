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
