use rand::Rng;

pub fn white_noise(buffer: &mut Vec<f32>) {
    let mut rng = rand::thread_rng();

    for sample in buffer.iter_mut() {
        *sample = rng.gen_range(-1.0 .. 1.0);
    }
}
