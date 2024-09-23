use super::FileExport;
use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};

pub struct WavExport {
    path: PathBuf,
}

impl WavExport {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
        }
    }
}

impl FileExport for WavExport {
    fn export(&self, buffer: &[u8]) -> std::io::Result<()> {
        let f = File::create(&self.path)?;
        let mut writer = BufWriter::new(f);
        writer.write(buffer)?;

        Ok(())
    }
}
