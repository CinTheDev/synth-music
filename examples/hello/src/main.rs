use std::str::FromStr;

use unnamed_music::{self, file_export::FileExport};

fn main() {
    println!("Hello example");

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let wav_export = unnamed_music::file_export::wav_export::WavExport::new(
        std::path::PathBuf::from_str("export/test.wav").unwrap(),
    );
    let mut buffer = [0_u16; 44100];

    let frequency = 440.0;
    let sample_rate = 44100;
    let amplitude = 0xFFFF as f32 * 0.1;

    for i in 0..buffer.len() {
        let time = i as f32 / sample_rate as f32;
        let value = (time * 2.0 * std::f32::consts::PI * frequency).sin() * amplitude;

        buffer[i] = value.round() as u16;
    }

    wav_export.export(bytemuck::cast_slice(&buffer)).unwrap();
}
