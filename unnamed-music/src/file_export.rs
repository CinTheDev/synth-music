pub mod wav_export;

pub trait FileExport {
    fn export(&self, buffer: &[u8]) -> std::io::Result<()>;
}
