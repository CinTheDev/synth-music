use super::note::{Note, ScaledValue, length, DynamicsFlag};
use length::Length;
use super::{SectionInfo, MusicTrack};
use super::TimeSignature;
use crate::instrument::Instrument;
use crate::file_export::export_info::{ExportTrack, Tone};
use std::time::Duration;
use std::ops::Range;

/// An implementation of MusicTrack with additional rules to ensure that
/// Measures are filled with notes correctly. Use this as the standard Track.
/// 
/// With the existance of measures, it's also possible to have a time signature
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
    measures: Vec<Measure<T>>,
    time_signature: TimeSignature,
    instrument: U,

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
        let intensity = self.current_intensity;
        let play_fraction = self.current_play_fraction;

        let dynamics_flag = self.next_note_dynamic_flag.take().unwrap_or(DynamicsFlag::None);

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
        let active_note = self.get_active_measure().notes.last_mut().unwrap();

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

    fn convert_to_export_track(&self, section_info: SectionInfo) -> ExportTrack<U> {
        let notes = self.arrange_notes();
        let mut tones = self.conversion_first_pass(&notes, section_info);
        Self::conversion_pass_dynamics(&notes, &mut tones);

        ExportTrack {
            tones,
            instrument: self.instrument.clone(),
        }
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
            measures: Vec::new(),
            time_signature,
            instrument,
            current_intensity: 1.0,
            current_play_fraction: 1.0,
            next_note_dynamic_flag: None,
        }
    }

    /// Place the end of a measure, which will automatically validate the
    /// completed measure. An error is returned if the measure is invalid.
    pub fn measure(&mut self) -> Result<&mut Measure<T>, &str> {
        let active_measure_valid = self.get_active_measure().assert_measure_bounds();

        if !active_measure_valid {
            return Err("Invalid measure bounds");
        }

        let new_measure = Measure::new(self.time_signature.clone());
        let valid_measure = self.active_measure.replace(new_measure).unwrap();

        self.measures.push(valid_measure);
        let last_index = self.measures.len() - 1;
        return Ok(&mut self.measures[last_index]);
    }

    fn get_active_measure(&mut self) -> &mut Measure<T> {
        self.active_measure.as_mut().unwrap()
    }

    fn arrange_notes(&self) -> Vec<(Note<T>, Length)> {
        let mut notes = Vec::new();

        let active_measure = self.active_measure.as_ref().unwrap();

        if ! active_measure.is_empty() {
            eprintln!("WARNING: Unvalidated measure, you probably forgot to call track.measure() at the end.");
        }

        for measure in &self.measures {
            let mut note_lengths = Vec::new();

            for note in &measure.notes {
                let position_in_measure =
                    Length::count_lengths(&note_lengths)
                    .unwrap_or(length::INVALID);

                notes.push(((*note).clone(), position_in_measure));
                note_lengths.push(note.length);
            }
        }

        return notes;
    }

    fn conversion_first_pass(&self, notes: &Vec<(Note<T>, Length)>, section_info: SectionInfo) -> Vec<Tone<U::ConcreteValue>> {
        let mut tones = Vec::new();

        for note in notes {
            let tone = self.generate_tone(&note, section_info);
            tones.push(tone);
        }

        return tones;
    }

    // WARNING: Assumes that notes align with tones
    // Fix if this doesn't apply anymore
    fn conversion_pass_dynamics(
        notes: &Vec<(Note<T>, Length)>,
        tones: &mut Vec<Tone<U::ConcreteValue>>
    ) {
        let mut i = 0;

        while let Some(notes_range) = Self::find_next_dynamics_change(&notes, i) {
            i = notes_range.end;
            Self::calculate_dynamics_over_notes(tones, notes_range);
        }
    }

    fn find_next_dynamics_change(notes: &Vec<(Note<T>, Length)>, start_index: usize) -> Option<Range<usize>> {
        use super::note::DynamicsFlag;

        let mut index_dynamics_start = None;

        for i in start_index..notes.len() {
            let (note, _) = &notes[i];

            if note.dynamics_flag == DynamicsFlag::StartChange {
                if index_dynamics_start.is_some() {
                    panic!("Doubled StartChange.");
                }
                
                index_dynamics_start = Some(i);
            }
            if note.dynamics_flag == DynamicsFlag::EndChange {
                if let Some(index_dynamics_start) = index_dynamics_start {
                    return Some(index_dynamics_start..(i + 1));
                }

                panic!("EndChange without preceding StartChange.");
            }
        }

        if index_dynamics_start.is_some() {
            panic!("StartChange without closing EndChange.");
        }

        return None;
    }

    fn calculate_dynamics_over_notes(tones: &mut Vec<Tone<U::ConcreteValue>>, range: Range<usize>) {
        let start_intensity = tones[range.start].intensity.start;
        let end_intensity = tones[range.end - 1].intensity.start;

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

    fn generate_tone(&self, note: &(Note<T>, Length), section_info: SectionInfo) -> Tone<U::ConcreteValue> {
        let mut concrete_values = Vec::new();

        let (note, position_in_measure) = note;

        for scaled_value in &note.values {
            let concrete_value = scaled_value.to_concrete_value(section_info.key);
            concrete_values.push(concrete_value);
        }

        let play_duration = note.get_duration(section_info.bpm);
        let tone_duration = play_duration.mul_f32(note.play_fraction);

        let beat_emphasis = self.get_beat_from_position(*position_in_measure);

        Tone {
            concrete_values,
            play_duration,
            tone_duration,
            intensity: note.intensity..note.intensity,
            beat_emphasis,
        }
    }

    fn get_beat_from_position(&self, position: Length) -> Option<f32> {
        let beats = self.time_signature.beats();

        let mut position_in_measure = Length::from_ticks(0);

        for beat in beats {
            if position == position_in_measure {
                return Some(*beat);
            }
            // Missed the beat
            if position.to_float() < position_in_measure.to_float() {
                return None;
            }

            position_in_measure += self.time_signature.beat_length();
        }

        return None;
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

    /// Override the time signature for this measure.
    pub fn override_time_signature(&mut self, time_signature: TimeSignature) -> &mut Self {
        self.time_signature = time_signature;
        self
    }
}
