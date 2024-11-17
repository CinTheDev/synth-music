use rand::Rng;

/// Fill a given buffer with white noise. The samples are randomly generated
/// in a range from -1.0 to 1.0.
pub fn white_noise(buffer: &mut Vec<f32>) {
    let mut rng = rand::thread_rng();

    for sample in buffer.iter_mut() {
        *sample = rng.gen_range(-1.0 .. 1.0);
    }
}
