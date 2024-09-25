pub mod instrument;
use instrument::Instrument;

enum MusicKey {
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
        unimplemented!()
    }
}

impl Section {
    pub fn new_instrument(&self) -> Instrument {
        Instrument {
            tracks: Vec::new(),
        }
    }
}
