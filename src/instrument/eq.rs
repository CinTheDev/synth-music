use super::SoundBuffer;
use realfft::RealFftPlanner;

/// Apply a lowpass filter to a periodic sound using a FFT. Only frequencies
/// lower than `frequency` will remain.
pub fn filter_fft_whole_lowpass(buffer: &mut SoundBuffer, frequency: f32) {
    filter_fft(buffer, |f: f32| -> f32 {
        if f < frequency {
            return 1.0;
        }
        else {
            return 0.0;
        }
    });
}

/// Apply a highpass filter to a periodic sound using a FFT. Only frequencies
/// higher than `frequency` will remain.
pub fn filter_fft_whole_highpass(buffer: &mut SoundBuffer, frequency: f32) {
    filter_fft(buffer, |f: f32| -> f32 {
        if f > frequency {
            return 1.0;
        }
        else {
            return 0.0;
        }
    });
}

/// Apply a bandpass filter to a periodic sound using a FFT. Only frequencies
/// in the specified range `frequency` will remain.
pub fn filter_fft_whole_bandpass(buffer: &mut SoundBuffer, frequency: std::ops::Range<f32>) {
    filter_fft(buffer, |f: f32| -> f32 {
        if f > frequency.start && f < frequency.end {
            return 1.0;
        }
        else {
            return 0.0;
        }
    })
}

/// Apply a FFT filter across the whole buffer, where the FFT length matches
/// the buffer length. More samples will result in a more precise filter.
/// 
/// This function will decompose the signal into its frequencies using a FFT,
/// and will multiply them with numbers given by `frequency_amplitude`. This
/// argument is a function that is given a frequency, and must return the
/// desired amplitude for that frequency. At the end the signal is reconstructed
/// using the altered frequencies.
pub fn filter_fft<F: Fn(f32) -> f32>(buffer: &mut SoundBuffer, frequency_amplitude: F) {
    let sample_rate = buffer.settings().sample_rate;
    let fft_len = buffer.samples.len();

    let mut planner = RealFftPlanner::new();
    let fft_forward = planner.plan_fft_forward(fft_len);
    let fft_inverse = planner.plan_fft_inverse(fft_len);

    let mut spectrum = fft_forward.make_output_vec();

    fft_forward.process(&mut buffer.samples, &mut spectrum).unwrap();

    let delta = sample_rate as f32 / fft_len as f32;

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
