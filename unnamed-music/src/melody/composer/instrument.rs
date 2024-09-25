pub mod note;
use note::Note;

pub struct Instrument<'a> {
    pub section: &'a super::Section<'a>,
    // TODO: Sound generation
    pub tracks: Vec<Track<'a>>,
}

pub struct Track<'a> {
    instrument: &'a Instrument<'a>,

    notes: Vec<Note>,

    current_length: note::Length,
    current_intensity: f32,
}

impl Instrument<'_> {
    pub fn new_track(&self) -> Track<'_> {
        Track {
            instrument: &self,
            notes: Vec::new(),
            current_length: note::Length::Quarter,
            current_intensity: 1.0,
        }
    }
}

impl Track<'_> {
    pub fn note(mut self, tone: note::Tone) -> Self {
        self.notes.push(Note {
            values: vec![tone],
            length: self.current_length,
            intensity: self.current_intensity,
        });
        self
    }

    pub fn set_length(mut self, length: note::Length) -> Self {
        self.current_length = length;
        self
    }

    pub fn set_intensity(mut self, intensity: f32) -> Self {
        self.current_intensity = intensity;
        self
    }
}
