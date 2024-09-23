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

    fn write_header(&self, writer: &mut BufWriter<File>, buffer_size: usize) -> std::io::Result<()> {
        use bytemuck::bytes_of;

        const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];
        const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];
        const FMT0: [u8; 4] = [b'f', b'm', b't', b' '];
        const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];

        let data_size: u32 = buffer_size.try_into().unwrap();
        let file_size = data_size + 44 - 8;

        let format_data_length: u16 = 16;
        // TODO: Make these configurable
        let num_channels: u16 = 1;
        let sample_rate: u32 = 44100;
        let bits_per_sample: u16 = 32;

        let sample_rate_bits_channels: u32 = sample_rate * bits_per_sample as u32 * num_channels as u32 / 8;

        writer.write(&RIFF)?;
        writer.write(bytes_of(&file_size))?;
        writer.write(&WAVE)?;
        writer.write(&FMT0)?;
        writer.write(bytes_of(&format_data_length))?;
        writer.write(bytes_of(&num_channels))?;
        writer.write(bytes_of(&sample_rate))?;
        writer.write(bytes_of(&sample_rate_bits_channels))?;
        writer.write(bytes_of(&(bits_per_sample * num_channels / 8)))?;
        writer.write(bytes_of(&bits_per_sample))?;
        writer.write(&DATA)?;
        writer.write(bytes_of(&data_size))?;

        Ok(())
    }
}

impl FileExport for WavExport {
    fn export(&self, buffer: &[u8]) -> std::io::Result<()> {
        let f = File::create(&self.path)?;
        let mut writer = BufWriter::new(f);

        self.write_header(&mut writer, buffer.len())?;
        writer.write(buffer)?;

        Ok(())
    }
}
