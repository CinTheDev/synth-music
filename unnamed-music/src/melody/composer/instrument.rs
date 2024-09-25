pub mod note;
use note::Note;

pub struct Instrument<'a> {
    section: &'a super::Section<'a>,
    // TODO: Sound generation
    pub tracks: Vec<Track<'a>>,
}

pub struct Track<'a> {
    instrument: &'a Instrument<'a>,

    notes: Vec<Note>,
}
