use super::FileExport;

pub struct WavExport {

}

impl FileExport for WavExport {
    fn export(buffer: &[u8]) -> Result<(), ()> {
        unimplemented!();
    }
}
