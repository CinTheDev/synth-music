use std::str::FromStr;

use unnamed_music::{self, file_export::FileExport};

fn main() {
    println!("Hello example");

    let wav_export = unnamed_music::file_export::wav_export::WavExport::new(
        std::path::PathBuf::from_str("test.txt").unwrap(),
    );
    let buffer = ['a' as u8; 16];
    wav_export.export(&buffer).unwrap();
}
