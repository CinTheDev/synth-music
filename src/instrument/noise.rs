use rand::Rng;

/// Fill a given buffer with white noise. The samples are randomly generated
/// in a range from -1.0 to 1.0.
pub fn white_noise(buffer: &mut Vec<f32>) {
    let mut rng = rand::thread_rng();

    for sample in buffer.iter_mut() {
        *sample = rng.gen_range(-1.0 .. 1.0);
    }
}

/// Fill a given buffer with pink noise. In pink noise, the amplitude of a
/// specific frequency is inversly proportional to the square root of the
/// frequency, which effectively makes higher frequencies quieter; it's similar
/// to red noise. This makes it sound less harsh.
/// 
/// **Not implemented yet**
pub fn pink_noise(buffer: &mut Vec<f32>) {
    let frequency_amplitude = |f: f32| {
        if f < 0.1 {
            return 0.0;
        }

        return 1.0 / f.sqrt();
    };

    custom_noise(buffer, frequency_amplitude);
}

/// Fill a given buffer with red noise (also called brown noise). In red noise,
/// the amplitude of a specific frequency is inversly proportional to the
/// frequency, which effectively makes higher frequencies quieter; it's similar
/// to pink noise. This makes it sound less harsh.
/// 
/// **Not implemented yet**
pub fn red_noise(buffer: &mut Vec<f32>) {
    let frequency_amplitude = |f: f32| {
        if f < 0.1 {
            return 0.0;
        }

        return 1.0 / f;
    };

    custom_noise(buffer, frequency_amplitude);
}

/// Fill a given buffer with blue noise. In blue noise, the amplitude of a
/// specific frequency is proportional to the square root of the frequency,
/// which effectively makes lower frequencies quieter; it's similar to purple
/// noise.
/// 
/// **Not implemented yet**
pub fn blue_noise(buffer: &mut Vec<f32>) {
    let frequency_amplitude = |f: f32| {
        let max_value = 20_000_f32.sqrt();
        return f.sqrt() / max_value;
    };

    custom_noise(buffer, frequency_amplitude);
}

/// Fill a given buffer with purple noise. In purple noise, the amplitude of
/// a specific frequency is proportional to the freqeuncy, which effectively
/// makes lower frequencies quieter; it's similar to blue noise.
/// 
/// **Not implemented yet**
pub fn purple_noise(buffer: &mut Vec<f32>) {
    let frequency_amplitude = |f: f32| {
        let max_value = 20_000.0;
        return f / max_value;
    };

    custom_noise(buffer, frequency_amplitude);
}

/// Fill the given buffer with filtered white noise. `frequency_amplitude` is a
/// function pointer; the given parameter is the frequency, and the function
/// shall return the amplitude of that frequency. The maximum amplitude is 1.0,
/// and the minimum amplitude is 0.0.
/// 
/// If e.g. the function is given the value `1000.0` and it returns `0.4`, then
/// the amplitude of the 1000Hz-portion of the noise will have 40% of the
/// amplitude of what it would have in white noise. If every frequency has
/// amplitude `1.0`. the white noise would remain unfiltered.
/// 
/// **Not implemented yet**
pub fn custom_noise(buffer: &mut Vec<f32>, frequency_amplitude: fn(f32) -> f32) {
    unimplemented!();
}
