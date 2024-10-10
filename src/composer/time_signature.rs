use super::note::Length;

#[derive(Clone)]
pub struct TimeSignature {
    pub measure_length: Length,
    // TODO: Beats
}

impl TimeSignature {
    pub const fn new(nominator: u8, denominator: u8) -> Self {
        // For now only powers of two for denominator are supported
        if ! denominator.is_power_of_two() {
            panic!("The denominator can only be a power of two.");
        }

        let subdivision = Self::what_power_of_two(denominator) - 1;
        let measure_length =
            Length::from_subdivisions(subdivision as u32)
            .const_mul(nominator as u32);

        Self {
            measure_length,
        }
    }

    pub fn is_measure_saturated(&self, lengths: Length) -> bool {
        return self.measure_length == lengths;
    }

    const fn what_power_of_two(mut value: u8) -> u8 {
        let mut result = 0;

        if value == 0  { panic!("Invalid input") };

        while value > 1 {
            value >>= 1;
            result += 1;
        }

        return result;
    }
}
