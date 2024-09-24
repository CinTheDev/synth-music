pub mod instrument;

use instrument::Instrument;

// Represents the whole music which can be exported
pub struct MusicPiece {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub bpm: u32,
    // TODO: Key
    // TODO: Time Signature

    pub instruments: Vec<Instrument>,
}
