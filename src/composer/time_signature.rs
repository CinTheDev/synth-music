use super::note::Length;

#[derive(Clone)]
pub struct TimeSignature {
    pub measure_length: Length,
    // TODO: Beats
}

impl TimeSignature {
    pub fn is_measure_saturated(&self, lengths: Length) -> bool {
        return self.measure_length == lengths;
    }
}
