// Composer imports
pub use crate::composer;

pub use composer::{
    music_key,
    note::length,
    Note,
    ScaledValue,

    TimeSignature,

    MusicTrack,
    UnboundTrack,
    measure_track::*,
};
pub use music_key::{MusicKey, KeyTonic, KeyType};

pub use crate::{notes, sequential_notes, section, composition};

// Instrument imports
pub use crate::instrument;
pub use instrument::{
    Instrument,
    predefined,
    noise,
    eq,
    curve,
};
pub use predefined::tet12;
pub use tet12::{TET12ScaledTone, TET12ConcreteTone};
pub use curve::linear_curve::LinearCurve;


// File-export imports
pub use crate::file_export;
pub use file_export::{
    FileExport,
    wav_export::WavExport,
    export_info
};

pub use export_info::{
    Tone,
    SoundBuffer,
    CompositionSettings,
    SectionInfo,
};
