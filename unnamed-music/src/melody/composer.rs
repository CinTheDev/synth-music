pub mod instrument;

enum MusicKey {
    C,
}

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
pub struct Composer {
    sections: Vec<Section>,
}

pub struct Section {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),
}
