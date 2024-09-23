use std::str::FromStr;

use unnamed_music::{self, file_export::FileExport};

fn main() {
    println!("Hello example");

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let wav_export = unnamed_music::file_export::wav_export::WavExport::new(
        std::path::PathBuf::from_str("export/test.txt").unwrap(),
    );
    let buffer = ['a' as u8; 16];
    wav_export.export(&buffer).unwrap();
}
