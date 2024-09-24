
// Represents the whole music which can be exported
pub struct MusicPiece {
    sections: Vec<Section>,
}

pub struct Section {
    bpm: u32,
    // TODO: Key
    // TODO: Time Signature

    instruments: Vec<Instrument>,
}
