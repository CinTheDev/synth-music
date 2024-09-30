pub use super::composer;
pub use composer::{Composition, Section, SectionInfo};
pub use composer::music_key::{MusicKey, KeyTonic, KeyType};
pub use composer::track::Track;
pub use composer::track::note;

pub use crate::{notes, sequential_notes};

pub use super::instrument;
pub use super::instrument::{Instrument, ToneInfo};

pub use super::export_info::ExportMusicPiece;
