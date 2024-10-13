use super::Length;

/// Represents a time signature from music for a Track.
/// 
/// It constraints how many notes of specific lengths can fit inside a single
/// measure, and it determines which beats are emphasized.
/// 
/// ```
/// # use synth_music::prelude::TimeSignature;
/// let four_four =
///     TimeSignature::new(4, 4)  // Create a time signature of type 4/4
///     .set_beat(0, 1.1)         // Strongly emphasize the first beat
///     .set_beat(2, 1.05);       // Weakly emphasize the third beat
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct TimeSignature {
    pub measure_length: Length,
    beat_length: Length,
    beat_intensities: Vec<f32>,
}

impl TimeSignature {
    /// Create a new time signature without specified emphasis. The time
    /// signature is of the form `nominator`/`denominator`.
    /// 
    /// Currently, only powers of two for the denominator are supported. Other
    /// values will result in a panic.
    pub fn new(nominator: u8, denominator: u8) -> Self {
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

    /// Set the emphasis of a specific beat. The standard value for all beats
    /// is `1.0`, a smaller value will weaken the beat, while a greater value
    /// will emphasize the beat.
    /// 
    /// Will panic if the specified index is beyond the amount of beats. The
    /// indexing works like an array, where `0` refers to the first beat, `1`
    /// to the second, etc...
    pub fn set_beat(mut self, index: usize, emphasis: f32) -> Self {
        self.beat_intensities[index] = emphasis;
        self
    }

    /// Checks if the given note length fits perfectly inside the measure.
    pub fn is_measure_saturated(&self, lengths: Length) -> bool {
        return self.measure_length == lengths;
    }

    /// Return a reference to the beat emphasis values.
    pub fn beats(&self) -> &Vec<f32> {
        &self.beat_intensities
    }

    /// Return the length of a single beat.
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

mod tests;
