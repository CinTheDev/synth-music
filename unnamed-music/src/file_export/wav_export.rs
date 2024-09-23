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

    fn write_header(&self, writer: &mut BufWriter<File>) -> std::io::Result<()> {
        const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];
        const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];
        const FMT0: [u8; 4] = [b'f', b'm', b't', b' '];
        const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];

        let data_size: u32 = 0;
        let file_size = data_size + 44 - 8;

        let format_data_length: u16 = 16;
        let num_channels: u16 = 1;
        let sample_rate: u32 = 44100;
        let bits_per_sample: u16 = 16;

        writer.write(&RIFF)?;
        writer.write(file_size: &[u8])?; // TODO: File size
        writer.write(&WAVE)?;
        writer.write(&FMT0)?;
        writer.write(format_data_length as &[u8])?; // TODO: Length of format data
        writer.write(num_channels as &[u8])?;// TODO: Number of channels
        writer.write(sample_rate as &[u8])?;// TODO: Sample rate
        writer.write(sample_rate * bits_per_sample.into() * num_channels.into() / 8 as &[u8])?; // TODO: Sample rate calculations
        writer.write(bits_per_sample * num_channels / 8 as &[u8])?;
        writer.write(bits_per_sample as &[u8])?;// TODO: Bits per sample
        writer.write(&DATA)?;
        writer.write(data_size as &[u8])?;// TODO: Data section size

        Ok(())
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
