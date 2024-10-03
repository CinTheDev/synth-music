// Composer imports
pub use crate::composer;
pub use composer::{SectionInfo, MusicTrack};

pub use composer::track::UnboundTrack;

pub use composer::note;
pub use composer::note::{Note, ScaledValue};

pub use composer::music_key::{MusicKey, KeyTonic, KeyType};

pub use crate::{notes, sequential_notes, section};

// Instrument imports
pub use crate::instrument;
pub use instrument::{Instrument, ToneInfo};
pub use instrument::predefined;
pub use predefined::tet12;

// File-export imports
pub use crate::file_export;
pub use file_export::{
    FileExport,
    wav_export::WavExport,
    export_info
};
