pub mod wav_writer;

pub trait FileWriter {
    fn export(buffer: &[u8], path: std::path::PathBuf);
}
