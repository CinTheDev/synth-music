use super::FileExport;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::str::FromStr;

pub struct WavExport {
    pub path: PathBuf,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
}

impl WavExport {
    fn write_header(&self, writer: &mut BufWriter<File>, buffer_size: usize) -> std::io::Result<()> {
        use bytemuck::bytes_of;

        const RIFF: [u8; 4] = [b'R', b'I', b'F', b'F'];
        const WAVE: [u8; 4] = [b'W', b'A', b'V', b'E'];
        const FMT0: [u8; 4] = [b'f', b'm', b't', b' '];
        const DATA: [u8; 4] = [b'd', b'a', b't', b'a'];

        let data_size: u32 = buffer_size.try_into().unwrap();
        let file_size = data_size + 44 - 8;

        let format_data_length: u32 = 16;
        let format_type: u16 = 1;
        // TODO: Make this configurable
        let num_channels: u16 = 1;

        let sample_rate_calculation: u32 =
            self.sample_rate * self.bits_per_sample as u32 * num_channels as u32 / 8;
        
        let bits_sample_calculation: u16 =
            self.bits_per_sample * num_channels / 8;

        writer.write(&RIFF)?;
        writer.write(bytes_of(&file_size))?;
        writer.write(&WAVE)?;
        writer.write(&FMT0)?;
        writer.write(bytes_of(&format_data_length))?;
        writer.write(bytes_of(&format_type))?;
        writer.write(bytes_of(&num_channels))?;
        writer.write(bytes_of(&self.sample_rate))?;
        writer.write(bytes_of(&sample_rate_calculation))?;
        writer.write(bytes_of(&bits_sample_calculation))?;
        writer.write(bytes_of(&self.bits_per_sample))?;
        writer.write(&DATA)?;
        writer.write(bytes_of(&data_size))?;

        Ok(())
    }
}

impl FileExport for WavExport {
    fn export(&self, buffer: Vec<f32>) -> std::io::Result<()> {
        let f = File::create(&self.path)?;
        let mut writer = BufWriter::new(f);

        // TODO: Generate chunks
        //let samples = buffer.generate_whole_buffer(self.sample_rate);

        self.write_header(&mut writer, buffer.len() * 2)?;
        let amplitude = 0xFFFF as f32 * 0.1;

        for sample in buffer {
            let val = (sample * amplitude).round() as i16;
            writer.write(bytemuck::bytes_of(&val))?;
        }

        Ok(())
    }
}

impl Default for WavExport {
    fn default() -> Self {
        Self {
            path: PathBuf::from_str("unnamed.wav").unwrap(),
            sample_rate: 44100,
            bits_per_sample: 16,
        }
    }
}
