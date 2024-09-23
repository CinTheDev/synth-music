use super::FileExport;

pub struct WAVWriter {

}

impl FileExport for WAVWriter {
    fn export(buffer: &[u8], path: std::path::PathBuf) -> Result<(), ()> {
        unimplemented!();
    }
}
