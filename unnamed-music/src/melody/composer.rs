pub mod instrument;
use instrument::Instrument;

pub enum MusicKey {
    C,
}

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
pub struct Composition {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),

    pub instruments: Vec<Instrument>,
}

impl Composition {
    pub fn to_export_piece(self) -> crate::melody::export_info::ExportMusicPiece {
        use crate::melody::export_info::*;
        let mut result = ExportMusicPiece::new();

        return result;
    }
}
