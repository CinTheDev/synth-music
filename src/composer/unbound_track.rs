use super::note::{DynamicsFlag, Length, Note, ScaledValue};
use super::{SectionInfo, MusicTrack};
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};
use std::ops::Range;
use std::time::Duration;

#[derive(Clone)]
pub struct UnboundTrack<T: ScaledValue, U: Instrument> {
    notes: Vec<Note<T>>,
    instrument: U,

    current_intensity: f32,
}

impl<T, U> MusicTrack<T, U> for UnboundTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    fn pause(&mut self, length: Length) -> &mut Note<T> {
        self.notes(length, vec![])
    }

    fn note(&mut self, length: Length, value: T) -> &mut Note<T> {
        self.notes(length, vec![value])
    }

    fn notes(&mut self, length: Length, values: Vec<T>) -> &mut Note<T> {
        self.notes.push(Note {
            values,
            length,
            intensity: self.current_intensity,
            ..Default::default()
        });

        let last_index = self.notes.len() - 1;
        return &mut self.notes[last_index];
    }

    fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }

    fn convert_to_export_track(self, section_info: SectionInfo) -> ExportTrack<U> {
        let mut tones = self.conversion_first_pass(section_info);

        self.conversion_pass_dynamics(&mut tones);

        ExportTrack {
            tones,
            instrument: self.instrument,
        }
    }
}

impl<T, U> UnboundTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    pub fn new(instrument: U) -> Self {
        Self {
            notes: Vec::new(),
            instrument,
            current_intensity: 1.0,
        }
    }

    fn conversion_first_pass(&self, section_info: SectionInfo) -> Vec<Tone<U::ConcreteValue>> {
        let mut tones = Vec::new();

        for note in &self.notes {
            let tone = Self::generate_tone(note, section_info);
            tones.push(tone);
        }

        return tones;
    }

    // WARNING: Assumes that notes align with tones
    // Fix if this doesn't apply anymore
    fn conversion_pass_dynamics(&self, tones: &mut Vec<Tone<U::ConcreteValue>>) {
        let mut i = 0;

        while let Some(notes_range) = Self::find_next_dynamics_change(&self.notes, i) {
            i = notes_range.end;
            Self::calculate_dynamics_over_notes(tones, notes_range);
        }
    }

    fn find_next_dynamics_change(notes: &Vec<Note<T>>, start_index: usize) -> Option<Range<usize>> {
        let mut index_dynamics_start = None;

        for i in start_index..notes.len() {
            let note = &notes[i];

            if note.dynamics_flag == DynamicsFlag::StartChange {
                if index_dynamics_start.is_some() {
                    panic!("Doubled StartChange.");
                }
                
                index_dynamics_start = Some(i);
            }
            if note.dynamics_flag == DynamicsFlag::EndChange {
                if let Some(index_dynamics_start) = index_dynamics_start {
                    return Some(index_dynamics_start..i);
                }
                else {
                    panic!("EndChange without preceding StartChange.");
                }
            }
        }

        if index_dynamics_start.is_some() {
            panic!("StartChange without closing EndChange.");
        }

        return None;
    }

    fn calculate_dynamics_over_notes(tones: &mut Vec<Tone<U::ConcreteValue>>, range: Range<usize>) {
        let start_intensity = tones[range.start].intensity.start;
        let end_intensity = tones[range.end].intensity.start;

        let mut time_delta = Duration::ZERO;

        for tone in &tones[range.clone()] {
            time_delta += tone.play_duration;
        }

        // We'll assume linear intensity increase
        // TODO: Make interpolation function custom

        let time_delta = time_delta; // Make immutable
        let mut current_time = Duration::ZERO;

        for tone in &mut tones[range] {
            let t = current_time.as_secs_f32() / time_delta.as_secs_f32();
            let intensity_at_start = Self::interpolate_intensity(start_intensity, end_intensity, t);
            
            current_time += tone.play_duration;
            let t = current_time.as_secs_f32() / time_delta.as_secs_f32();
            let intensity_at_end = Self::interpolate_intensity(start_intensity, end_intensity, t);

            tone.intensity = intensity_at_start..intensity_at_end;
        }
    }

    fn interpolate_intensity(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    fn generate_tone(note: &Note<T>, section_info: SectionInfo) -> Tone<U::ConcreteValue> {
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
            intensity: note.intensity..note.intensity,
        }
    }
}
