use super::UnboundTrack;

use super::{Note, ScaledValue, length, Length};
use super::note::DynamicsFlag;

use super::{TimeSignature, SectionInfo, MusicTrack};
use super::ExportTrack;

use crate::instrument::Instrument;

/// An implementation of MusicTrack with additional rules to ensure that
/// Measures are filled with notes correctly. Use this as the standard Track.
/// 
/// With the existence of measures, it's also possible to have a time signature
/// assigned to the track, which can even automatically emphasize specific beats
/// (e.g. the first beat).
/// 
/// Read the front page of the crate for examples on how to use this.
pub struct MeasureTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    active_measure: Option<Measure<T>>,
    unbound_track: UnboundTrack<T, U>,
    time_signature: TimeSignature,

    current_intensity: f32,
    current_play_fraction: f32,

    next_note_dynamic_flag: Option<DynamicsFlag>,
}

/// A single measure; managed by `MeasureTrack`
pub struct Measure<T: ScaledValue> {
    time_signature: TimeSignature,
    notes: Vec<Note<T>>,
}

impl<T, U> MusicTrack<T, U> for MeasureTrack<T, U>
where
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    fn notes(&mut self, length: Length, values: Vec<T>) -> &mut Note<T> {
        let play_fraction = self.current_play_fraction;
        
        let dynamics_flag = self.next_note_dynamic_flag.take().unwrap_or(DynamicsFlag::None);
        
        let current_measure_length = self.get_active_measure().get_total_length();
        let beat_emphasis = self.get_beat_from_position(current_measure_length);

        let intensity = self.current_intensity * beat_emphasis;
        
        let active_measure = self.get_active_measure();

        active_measure.notes.push(Note {
            values,
            length,
            intensity,
            play_fraction,
            dynamics_flag,
            ..Default::default()
        });

        let last_index = active_measure.notes.len() - 1;
        return &mut active_measure.notes[last_index];
    }

    fn note(&mut self, length: Length, value: T) -> &mut Note<T> {
        self.notes(length, vec![value])
    }

    fn pause(&mut self, length: Length) -> &mut Note<T> {
        self.notes(length, vec![])
    }

    fn start_dynamic_change(&mut self) {
        self.next_note_dynamic_flag = Some(DynamicsFlag::StartChange);
    }

    fn end_dynamic_change(&mut self, intensity: f32) {
        let active_note = self.get_active_note();

        active_note.dynamics_flag = DynamicsFlag::EndChange;
        active_note.intensity = intensity;
        self.current_intensity = intensity;
    }

    fn set_intensity(&mut self, intensity: f32) {
        self.current_intensity = intensity;
    }

    fn set_play_fraction(&mut self, play_fraction: f32) {
        self.current_play_fraction = play_fraction;
    }

    fn get_active_note(&mut self) -> &mut Note<T> {
        let active_measure_empty = self.get_active_measure().is_empty();

        if active_measure_empty {
            return self.unbound_track.get_active_note().unwrap();
        }

        let active_measure = self.get_active_measure();
        return active_measure.notes.last_mut().unwrap();
    }

    fn convert_to_export_track(&self, section_info: SectionInfo) -> ExportTrack<U> {
        self.unbound_track.convert_to_export_track(section_info)
    }
}

impl<T, U> MeasureTrack<T, U>
where 
    T: ScaledValue<ConcreteValue = U::ConcreteValue>,
    U: Instrument,
{
    /// Create a new track with the given instrument and time signature
    pub fn new(instrument: U, time_signature: TimeSignature) -> Self {
        Self {
            active_measure: Some(Measure::new(time_signature.clone())),
            unbound_track: UnboundTrack::new(instrument),
            time_signature,
            current_intensity: 0.5,
            current_play_fraction: 1.0,
            next_note_dynamic_flag: None,
        }
    }

    /// Place the end of a measure, which will automatically validate the
    /// completed measure. An error is returned if the measure is invalid.
    pub fn measure(&mut self) -> Result<(), &str> {
        let active_measure_valid = self.get_active_measure().assert_measure_bounds();

        if !active_measure_valid {
            return Err("Invalid measure bounds");
        }

        let new_measure = Measure::new(self.time_signature.clone());
        let valid_measure = self.active_measure.replace(new_measure).unwrap();

        self.unbound_track.append_notes(&valid_measure.notes);

        return Ok(());
    }

    pub fn get_active_measure(&mut self) -> &mut Measure<T> {
        self.active_measure.as_mut().unwrap()
    }

    fn get_beat_from_position(&self, position: Length) -> f32 {
        let beats = self.time_signature.beats();

        let mut position_in_measure = length::ZERO;

        for beat in beats {
            if position == position_in_measure {
                return *beat;
            }
            // Missed the beat
            if position.to_float() < position_in_measure.to_float() {
                return self.time_signature.offbeat_intensity();
            }

            position_in_measure += self.time_signature.beat_length();
        }

        return self.time_signature.offbeat_intensity();
    }
}

impl<T: ScaledValue> Measure<T> {
    fn new(time_signature: TimeSignature) -> Self {
        Self {
            time_signature,
            notes: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    fn assert_measure_bounds(&self) -> bool {
        let mut all_lengths = Vec::new();

        for note in &self.notes {
            all_lengths.push(note.length);
        }

        let total_length = Length::count_lengths(&all_lengths).unwrap();

        return self.time_signature.is_measure_saturated(total_length);
    }

    fn get_total_length(&self) -> Length {
        let mut note_lengths = Vec::new();

        for note in &self.notes {
            note_lengths.push(note.length);
        }

        return
            Length::count_lengths(&note_lengths)
            .unwrap_or(length::INVALID);
    }

    /// Override the time signature for this measure.
    pub fn override_time_signature(&mut self, time_signature: TimeSignature) -> &mut Self {
        self.time_signature = time_signature;
        self
    }
}

mod tests;
