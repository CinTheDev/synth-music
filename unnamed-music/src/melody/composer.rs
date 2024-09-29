pub mod track;
use track::Track;
use track::note::Note;
use super::export_info::*;

use std::time::Duration;

const DEFAULT_FADE_IN: Duration = Duration::from_millis(2);
const DEFAULT_FADE_OUT: Duration = Duration::from_millis(2);

#[derive(Clone, Copy)]
pub enum MusicKeyBase {
    C,
    Csharp,
    Cflat,
    D,
    Dflat,
    E,
    Eflat,
    F,
    Fsharp,
    G,
    Gflat,
    A,
    Aflat,
    B,
    Bflat,
}
#[derive(Clone, Copy)]
pub enum MusicKeyType {
    Major,
    Minor,
}
#[derive(Clone, Copy)]
pub struct MusicKey {
    pub base: MusicKeyBase,
    pub key_type: MusicKeyType,
}

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

        let play_duration = note.get_duration(section_info.bpm);
        let tone_duration = play_duration.mul_f32(note.play_fraction);

        Tone {
            concrete_values: todo!(),
            play_duration,
            tone_duration,
            intensity: note.intensity,
        }
        /*
        let mut frequencies = Vec::new();

        for tone in &note.values {
            let base_frequency = modify_frequency(
                get_note_base_frequency(*tone, section_info.key.key_type),
                note.semitones_offset,
            );
            let keyed_frequency = transpose_from_base(base_frequency, section_info.key);
            
            frequencies.push(keyed_frequency);
        }

        let play_duration = note.get_duration(section_info.bpm);
        let tone_duration = play_duration.mul_f32(note.play_fraction);

        Tone {
            frequencies,
            play_duration,
            tone_duration,
            intensity: note.intensity,
            fade_in: DEFAULT_FADE_IN,
            fade_out: DEFAULT_FADE_OUT,
        }
        */
    }
}

fn get_note_base_frequency(tone: (track::note::Tone, i32), key_type: MusicKeyType) -> f32 {
    // Major keys are here in C
    // Minor keys in A
    match key_type {
        MusicKeyType::Major => get_note_base_frequency_major(tone),
        MusicKeyType::Minor => get_note_base_frequency_minor(tone),
    }
}

fn transpose_from_base(frequency: f32, key: MusicKey) -> f32 {
    match key.key_type {
        MusicKeyType::Major => transpose_major(frequency, key.base),
        MusicKeyType::Minor => transpose_minor(frequency, key.base),
    }
}

fn transpose_major(frequency: f32, key: MusicKeyBase) -> f32 {
    let offset = match key {
        MusicKeyBase::Gflat => -6,
        MusicKeyBase::G => -5,
        MusicKeyBase::Aflat => -4,
        MusicKeyBase::A => -3,
        MusicKeyBase::Bflat => -2,
        MusicKeyBase::B |
        MusicKeyBase::Cflat => -1,
        MusicKeyBase::C => 0,
        MusicKeyBase::Csharp |
        MusicKeyBase::Dflat => 1,
        MusicKeyBase::D => 2,
        MusicKeyBase::Eflat => 3,
        MusicKeyBase::E => 4,
        MusicKeyBase::F => 5,
        MusicKeyBase::Fsharp => 6,
    };

    modify_frequency(frequency, offset)
}

fn transpose_minor(frequency: f32, key: MusicKeyBase) -> f32 {
    let offset = match key {
        MusicKeyBase::Eflat => -6,
        MusicKeyBase::E => -5,
        MusicKeyBase::F => -4,
        MusicKeyBase::Fsharp |
        MusicKeyBase::Gflat => -3,
        MusicKeyBase::G => -2,
        MusicKeyBase::Aflat => -1,
        MusicKeyBase::A => 0,
        MusicKeyBase::Bflat => 1,
        MusicKeyBase::B => 2,
        MusicKeyBase::Cflat => 3,
        MusicKeyBase::C => 4,
        MusicKeyBase::Csharp |
        MusicKeyBase::Dflat => 5,
        MusicKeyBase::D => 6,
    };

    modify_frequency(frequency, offset)
}

fn get_note_base_frequency_major(tone: (track::note::Tone, i32)) -> f32 {
    use track::note::Tone;
    let base_frequency = match tone.0 {
        Tone::First => get_frequency_from_a4(-9),
        Tone::Second => get_frequency_from_a4(-7),
        Tone::Third => get_frequency_from_a4(-5),
        Tone::Fourth => get_frequency_from_a4(-4),
        Tone::Fith => get_frequency_from_a4(-2),
        Tone::Sixth => get_frequency_from_a4(0),
        Tone::Seventh => get_frequency_from_a4(2),
    };

    base_frequency * 2_f32.powi(tone.1 - 4)
}

fn get_note_base_frequency_minor(tone: (track::note::Tone, i32)) -> f32 {
    use track::note::Tone;
    let base_frequency = match tone.0 {
        Tone::First => get_frequency_from_a4(0),
        Tone::Second => get_frequency_from_a4(2),
        Tone::Third => get_frequency_from_a4(3),
        Tone::Fourth => get_frequency_from_a4(5),
        Tone::Fith => get_frequency_from_a4(7),
        Tone::Sixth => get_frequency_from_a4(8),
        Tone::Seventh => get_frequency_from_a4(10),
    };

    base_frequency * 2_f32.powi(tone.1 - 4)
}

fn get_frequency_from_a4(semitones: i32) -> f32 {
    2_f32.powf(semitones as f32 / 12.0) * 440.0
}

fn modify_frequency(frequency: f32, semitones: i32) -> f32 {
    2_f32.powf(semitones as f32 / 12.0) * frequency
}
