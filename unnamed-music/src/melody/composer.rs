pub mod track;
pub mod music_key;

use track::Track;
use track::note::Note;
use music_key::MusicKey;
use super::export_info::*;

// A helper struct to compose a piece. At the end, an ExportMusicPiece can be
// generated from it.
pub struct Composition {
    pub sections: Vec<Section>,
}

#[derive(Clone, Copy)]
pub struct SectionInfo {
    pub bpm: f32,
    pub key: MusicKey,
    pub time_signature: (u8, u8),
}

#[derive(Clone)]
pub struct Section {
    pub info: SectionInfo,
    pub tracks: Vec<Track>,
}

impl Composition {
    pub fn to_export_piece(self) -> ExportMusicPiece {
        let mut result = ExportMusicPiece::new();

        for section in self.sections {
            let export_section = Self::generate_export_section(section);
            result.sections.push(export_section);
        }

        return result;
    }

    fn generate_export_section(section: Section) -> ExportSection {
        let mut export_section = ExportSection::new();

        for track in section.tracks {
            let export_track = Self::generate_export_track(track, section.info);
            export_section.tracks.push(export_track);
        }

        return export_section;
    }

    fn generate_export_track(track: Track, section_info: SectionInfo) -> ExportTrack {
        let (notes, instrument) = track.into_parts();

        let mut export_track = ExportTrack::new(instrument);

        for note in notes {
            let tone = Self::generate_tone(note, section_info);
            export_track.tones.push(tone);
        }

        return export_track;
    }

    fn generate_tone(note: Note, section_info: SectionInfo) -> Tone {
        let mut concrete_values = Vec::new();

        for scaled_value in &note.values {
            let concrete_value = ConcreteValue(scaled_value.get_concrete_value(section_info.key));
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
