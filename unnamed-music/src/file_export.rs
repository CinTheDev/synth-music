pub mod wav_export;

pub trait FileExport {
    fn export(buffer: &[u8], path: std::path::PathBuf) -> Result<(), ()>;
}
