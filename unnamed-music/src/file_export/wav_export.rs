use super::FileExport;
use std::path::PathBuf;

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
    fn export(buffer: &[u8]) -> Result<(), ()> {
        unimplemented!();
    }
}
