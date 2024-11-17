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
    unimplemented!();
}

/// Fill a given buffer with red noise. In red noise, the amplitude of a
/// specific frequency is inversly proportional to the frequency, which
/// effectively makes higher frequencies quieter; it's similar to pink noise.
/// This makes it sound less harsh.
/// 
/// **Not implemented yet**
pub fn red_noise(buffer: &mut Vec<f32>) {
    unimplemented!();
}

/// Fill a given buffer with blue noise. In blue noise, the amplitude of a
/// specific frequency is proportional to the square root of the frequency,
/// which effectively makes lower frequencies quieter; it's similar to purple
/// noise.
/// 
/// **Not implemented yet**
pub fn blue_noise(buffer: &mut Vec<f32>) {
    unimplemented!();
}

/// Fill a given buffer with purple noise. In purple noise, the amplitude of
/// a specific frequency is proportional to the freqeuncy, which effectively
/// makes lower frequencies quieter; it's similar to blue noise.
/// 
/// **Not implemented yet**
pub fn purple_noise(buffer: &mut Vec<f32>) {
    unimplemented!();
}
