pub mod wav_export;

pub trait FileExport {
    fn export(&self, buffer: MusicBuffer) -> std::io::Result<()>;
}

struct MusicBuffer {
    
}
