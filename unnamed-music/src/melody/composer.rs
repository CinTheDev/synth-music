pub mod instrument;
use instrument::Instrument;

enum MusicKey {
    C,
}

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
pub struct Composition<'a> {
    pub sections: Vec<Section<'a>>,
}

pub struct Section<'a> {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),

    pub instruments: Vec<Instrument<'a>>,
}

impl Composition<'_> {
    pub fn to_export_piece(self) -> crate::melody::export_info::ExportMusicPiece {
        unimplemented!()
    }
}

impl Section<'_> {
    pub fn new_instrument(&self) -> Instrument<'_> {
        Instrument {
            section: &self,
            tracks: Vec::new(),
        }
    }
}
