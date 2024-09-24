pub mod wav_export;

use std::time::Duration;
use super::melody::MusicPiece;

pub trait FileExport {
    fn export(&self, buffer: MusicBuffer) -> std::io::Result<()>;
}

pub struct MusicBuffer {
    piece: MusicPiece,
}

impl MusicBuffer {
    pub fn new(piece: MusicPiece) -> Self {
        Self {
            piece,
        }
    }

    /*
    pub fn generate_buffer(&self, start_time: Duration, end_time: Duration) -> Result<&[u8], &'static str> {
        Ok(&self.buffer)
    }
    */
    pub fn generate_whole_buffer(&self, sample_rate: u32) -> Vec<f32> {
        let mut buffer: Vec<f32> = Vec::new();

        for section in &self.piece.sections {
            
        }

        return buffer;
    }
}
