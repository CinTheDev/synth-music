use std::time::Duration;

pub mod wav_export;

pub trait FileExport {
    fn export(&self, buffer: MusicBuffer) -> std::io::Result<()>;
}

pub struct MusicBuffer {

}

impl MusicBuffer {
    pub fn generate_buffer(&self, start_time: Duration, end_time: Duration) -> Result<Vec<u8>, &'static str> {
        Err("Unimplemented")
    }
}
