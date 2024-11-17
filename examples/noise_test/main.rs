use synth_music::prelude::*;

fn main() {
    let settings = CompositionSettings {
        sample_rate: 48000,
    };

    let sample_num = settings.sample_rate as usize;

    let empty_buffer = SoundBuffer::from_parts(
        vec![0.0; sample_num],
        sample_num,
        settings,
    );

    let silence_inbetween = SoundBuffer::from_parts(
        vec![0.0; sample_num / 2],
        sample_num / 2,
        settings,
    );

    let mut white_noise = empty_buffer.clone();
    let mut pink_noise = empty_buffer.clone();
    let mut red_noise = empty_buffer.clone();
    let mut blue_noise = empty_buffer.clone();
    let mut purple_noise = empty_buffer.clone();

    let mut test_1 = SoundBuffer::from_parts(
        vec![1.0; sample_num],
        sample_num,
        settings,
    );
    let mut test_2 = SoundBuffer::from_parts(
        vec![-1.0; sample_num],
        sample_num,
        settings,
    );
    eq::make_seamless(&mut test_1.samples, &mut test_2.samples, 10);

    noise::white_noise(&mut white_noise.samples);
    noise::pink_noise(&mut pink_noise);
    noise::red_noise(&mut red_noise);
    noise::blue_noise(&mut blue_noise);
    noise::purple_noise(&mut purple_noise);

    let out_buffer = composition!(
        white_noise,
        silence_inbetween,
        pink_noise,
        silence_inbetween,
        red_noise,
        silence_inbetween,
        blue_noise,
        silence_inbetween,
        purple_noise,
        silence_inbetween,
        test_1,
        test_2,
    );

    export(out_buffer);
}

fn export(buffer: SoundBuffer) {
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let wav_export = WavExport {
        path: PathBuf::from("export/Noise_Test.wav"),
        ..Default::default()
    };

    wav_export.export(buffer).unwrap();
}
