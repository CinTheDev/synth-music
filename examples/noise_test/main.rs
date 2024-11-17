use synth_music::prelude::*;

fn main() {

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
