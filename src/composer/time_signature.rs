use super::note::Length;

//pub const FOUR_FOUR: TimeSignature = TimeSignature::new(4, 4);
//pub const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
//pub const TWO_FOUR: TimeSignature = TimeSignature::new(2, 4);

//pub const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);
//pub const NINE_EIGHT: TimeSignature = TimeSignature::new(9, 8);
//pub const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);

#[derive(Clone)]
pub struct TimeSignature {
    pub measure_length: Length,
    beat_length: Length,
    beat_intensities: Vec<f32>,
}

impl TimeSignature {
    pub fn new(nominator: u8, denominator: u8) -> Self {
        // For now only powers of two for denominator are supported
        if ! denominator.is_power_of_two() {
            panic!("The denominator can only be a power of two.");
        }

        let subdivision = Self::what_power_of_two(denominator);

        let beat_length = Length::from_subdivisions(subdivision.into());
        let measure_length = beat_length * nominator.into();
        let beat_intensities = vec![1.0; nominator.into()];

        Self {
            measure_length,
            beat_length,
            beat_intensities,
        }
    }

    pub fn set_beat(mut self, index: usize, emphasis: f32) -> Self {
        self.beat_intensities[index] = emphasis;
        self
    }

    pub fn is_measure_saturated(&self, lengths: Length) -> bool {
        return self.measure_length == lengths;
    }

    pub fn beats(&self) -> &Vec<f32> {
        &self.beat_intensities
    }

    pub fn beat_length(&self) -> Length {
        self.beat_length
    }

    fn what_power_of_two(mut value: u8) -> u8 {
        let mut result = 0;

        if value == 0  { panic!("Invalid input") };

        while value > 1 {
            value >>= 1;
            result += 1;
        }

        return result;
    }
}
