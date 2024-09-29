use unnamed_music::melody::prelude::*;

fn main() {
    println!("Hello example");
}

fn export(export_piece: ExportMusicPiece) {
    use unnamed_music::file_export::*;
    use wav_export::WavExport;
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let music_buffer = MusicBuffer::new(export_piece);
    let exporter = WavExport {
        path: PathBuf::from("export/test.wav"),
        sample_rate: 44100,
        ..Default::default()
    };

    exporter.export(music_buffer).unwrap();
}
