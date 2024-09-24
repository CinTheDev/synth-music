use std::time::Duration;

pub mod wav_export;

pub trait FileExport {
    fn export(&self, buffer: MusicBuffer) -> std::io::Result<()>;
}

pub struct MusicBuffer {
    // TODO: Generate buffer instead of storing
    buffer: Vec<u8>,
}

impl MusicBuffer {
    pub fn generate_buffer(&self, start_time: Duration, end_time: Duration) -> Result<&[u8], &'static str> {
        Ok(&self.buffer)
    }
}
