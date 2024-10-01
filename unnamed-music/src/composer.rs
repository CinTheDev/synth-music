pub mod note;
pub mod track;
pub mod music_key;

use track::Track;
use note::{Note, ScaledValue};
use music_key::MusicKey;

use crate::file_export::export_info::*;
use crate::instrument::Instrument;

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
pub struct Composition<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    pub sections: Vec<Section<T, U>>,
}

#[derive(Clone, Copy)]
pub struct SectionInfo {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),
}

#[derive(Clone)]
pub struct Section<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    pub info: SectionInfo,
    pub tracks: Vec<Track<T, U>>,
}

impl<T, U> Composition<T, U>
where 
    T: ScaledValue,
    U: Instrument<ConcreteValue = T::ConcreteValue>,
{
    pub fn to_export_piece(self) -> ExportMusicPiece<U> {
        let mut result = ExportMusicPiece::new();

        for section in self.sections {
            let export_section = Self::generate_export_section(section);
            result.sections.push(export_section);
        }

        return result;
    }

    fn generate_export_section(section: Section<T, U>) -> ExportSection<U> {
        let mut export_section = ExportSection::new();

        for track in section.tracks {
            let export_track = Self::generate_export_track(track, section.info);
            export_section.tracks.push(export_track);
        }

        return export_section;
    }

    fn generate_export_track(track: Track<T, U>, section_info: SectionInfo) -> ExportTrack<U> {
        let (notes, instrument) = track.into_parts();

        let mut export_track = ExportTrack::new(instrument);

        for note in notes {
            let tone = Self::generate_tone(note, section_info);
            export_track.tones.push(tone);
        }

        return export_track;
    }

    fn generate_tone(note: Note<T>, section_info: SectionInfo) -> Tone<U::ConcreteValue> {
        let mut concrete_values = Vec::new();

        for scaled_value in &note.values {
            let concrete_value = scaled_value.to_concrete_value(section_info.key);
            concrete_values.push(concrete_value);
        }

        let play_duration = note.get_duration(section_info.bpm);
        let tone_duration = play_duration.mul_f32(note.play_fraction);

        Tone {
            concrete_values,
            play_duration,
            tone_duration,
            intensity: note.intensity,
        }
    }
}