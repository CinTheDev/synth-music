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
}

impl Instrument<'_> {
    pub fn new_track(&self) -> Track<'_> {
        Track {
            instrument: &self,
            notes: Vec::new(),
        }
    }
}
