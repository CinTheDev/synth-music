use super::SoundBuffer;
use realfft::RealFftPlanner;

pub fn filter_fft_whole_lowpass(buffer: &mut SoundBuffer, frequency: f32) {
    filter_fft_whole(buffer, |f: f32| -> f32 {
        if f < frequency {
            return 1.0;
        }
        else {
            return 0.0;
        }
    });
}

pub fn filter_fft_whole_highpass(buffer: &mut SoundBuffer, frequency: f32) {
    filter_fft_whole(buffer, |f: f32| -> f32 {
        if f > frequency {
            return 1.0;
        }
        else {
            return 0.0;
        }
    });
}

/// Apply a FFT filter across the whole buffer, where the FFT length matches
/// the buffer length. Only do this for periodic signals (e.g. tones).
/// 
/// This function will decompose the signal into its frequencies using a FFT,
/// and will multiply them with numbers given by `frequency_amplitude`. This
/// argument is a function that is given a frequency, and must return the
/// desired amplitude for that frequency. At the end the signal is reconstructed
/// using the altered frequencies.
pub fn filter_fft_whole<F: Fn(f32) -> f32>(buffer: &mut SoundBuffer, frequency_amplitude: F) {
    let sample_rate = buffer.settings().sample_rate;
    filter_fft_part(&mut buffer.samples, sample_rate, &frequency_amplitude);
}

/// Apply a FFT filter across the whole buffer, where the FFT length is given
/// as a parameter. The FFT window is overlapped by `window_overlap` samples.
/// Without overlap there will be noticable seams between the FFT windows, and
/// higher overlap will make the seams less apparent.
/// 
/// This function is considered unfinished as it does not transition between
/// FFT windows seamlessly even with high overlap. Please use `filter_fft_whole`
/// unless you really need a specific fft window size.
/// 
/// TODO: Fix the overlap problem.
pub fn filter_fft_sized<F: Fn(f32) -> f32>(
    buffer: &mut SoundBuffer,
    frequency_amplitude: F,
    fft_len: usize,
    window_overlap: usize,
) {
    let sample_rate = buffer.settings().sample_rate;

    let mut index_start = 0;
    let mut index_end = fft_len;

    let mut part_results = Vec::new();

    while index_end <= buffer.samples.len() {
        let samples = &buffer.samples[index_start .. index_end];
        let mut part_buffer = samples.to_vec();
        filter_fft_part(&mut part_buffer, sample_rate, &frequency_amplitude);
        
        let offset = fft_len - window_overlap;
        index_start += offset;
        index_end += offset;

        part_results.push(SoundBuffer::from_parts(
            part_buffer,
            offset,
            buffer.settings(),
        ));
    }

    index_end = buffer.samples.len();

    let samples = &buffer.samples[index_start .. index_end];
    let mut part_buffer = samples.to_vec();
    filter_fft_part(&mut part_buffer, sample_rate, &frequency_amplitude);

    part_results.push(SoundBuffer::from_parts(
        part_buffer,
        index_end - index_start,
        buffer.settings(),
    ));

    *buffer = SoundBuffer::new(buffer.settings());

    for part_buffer in part_results {
        buffer.transition(part_buffer);
    }
}

fn filter_fft_part<F: Fn(f32) -> f32>(
    buffer: &mut [f32],
    sample_rate: u32,
    frequency_amplitude: &F,
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
