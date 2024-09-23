pub mod wav_export;

pub trait FileExport {
    fn export(buffer: &[u8]) -> Result<(), ()>;
}
