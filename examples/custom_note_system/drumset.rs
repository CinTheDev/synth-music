use rand::Rng;
use synth_music::prelude::*;
use std::time::Duration;

// Specify possible drumset actions
#[derive(Clone, Copy)]
pub enum DrumsetAction {
    Bass,
    Snare,
    HiHat,
}

// !! This is important
// This will convert the ScaledValue into a ConcreteValue, which is then
// given to the instrument. It's also possible to not perform any conversion
// at all and to just return a copy of self. (But this still needs to be
// implemented to be placed on tracks.)
impl ScaledValue for DrumsetAction {
    type ConcreteValue = Self;

    // The music key is availabe for conversion, but in this example it is not needed.
    fn to_concrete_value(&self, _key: MusicKey) -> Self::ConcreteValue {
        self.to_owned()
    }
}

#[derive(Clone, Copy)]
pub struct Drumset {
    play_duration: Duration
}

impl Drumset {
    pub fn new() -> Self {
        Self {
            play_duration: Duration::from_secs_f32(0.3),
        }
    }

    pub fn bass(&self, buffer_info: &BufferInfo, buffer: &mut Vec<f32>) {
        let frequency = 50.0;

        for i in 0..buffer.len() {
            let time = buffer_info.time_from_index(i);
            let value = predefined::sine_wave(frequency, time);
            buffer[i] = value * self.decay(time);
        }
    }

    pub fn noised_tone(&self, buffer_info: &BufferInfo, buffer: &mut Vec<f32>, action: &DrumsetAction) {
        let frequency_range = match action {
            DrumsetAction::Snare => 20.0 .. 20000.0,
            DrumsetAction::HiHat => 10000.0 .. 20000.0,
            
            _ => panic!("Invalid action in noised_tone"),
        };
        
        Self::generate_white_noise(buffer);
        Self::filter(buffer, buffer_info, frequency_range);
    }

    pub fn generate_white_noise(buffer: &mut Vec<f32>) {
        let mut rng = rand::thread_rng();

        for sample in buffer.iter_mut() {
            *sample = rng.gen_range(-1.0..1.0);
        }
    }

    pub fn filter(buffer: &mut Vec<f32>, buffer_info: &BufferInfo, frequency: std::ops::Range<f32>) {
        // Let's apply simple filter first
        use biquad::*;

        let f_lower = frequency.start.hz();
        let f_upper = frequency.end.hz();
        //let f_cutoff = 1000.hz();
        let f_sample = buffer_info.sample_rate.hz();

        let coeffs_lp = Coefficients::<f32>::from_params(
            Type::LowPass,
            f_sample,
            f_upper,
            Q_BUTTERWORTH_F32,
        ).unwrap();
        let coeffs_hp = Coefficients::<f32>::from_params(
            Type::HighPass,
            f_sample,
            f_lower,
            Q_BUTTERWORTH_F32,
        ).unwrap();

        let mut biquad_lp = DirectForm1::<f32>::new(coeffs_lp);
        let mut biquad_hp = DirectForm1::<f32>::new(coeffs_hp);
        
        for sample in buffer.iter_mut() {
            *sample = biquad_lp.run(*sample);
            *sample = biquad_hp.run(*sample);
        }
    }

    fn mix_buffers(mut a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
        for i in 0..a.len() {
            a[i] += b[i];
        }

        return a;
    }

    fn decay(&self, time: Duration) -> f32 {
        let factor = 5.0 / self.play_duration.as_secs_f32();
        0.5_f32.powf(time.as_secs_f32() * factor)
    }
}

impl Instrument for Drumset {
    type ConcreteValue = DrumsetAction;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let target_samples = (self.play_duration.as_secs_f32() * buffer_info.sample_rate as f32)
            .ceil() as usize;

        let mut result = vec![0_f32; target_samples];
        
        for action in &tones.concrete_values {
            match action {
                DrumsetAction::Bass => {
                    let mut buffer = vec![0_f32; target_samples];
                    self.bass(&buffer_info, &mut buffer);
                    result = Self::mix_buffers(result, buffer);
                },

                _ => {
                    let mut buffer = vec![0_f32; target_samples];
                    self.noised_tone(&buffer_info, &mut buffer, action);
                    result = Self::mix_buffers(result, buffer);
                }
            }
        }

        let intensity = tones.intensity.start * tones.beat_emphasis.unwrap_or(1.0);

        for value in result.iter_mut() {
            *value *= intensity;
        }

        InstrumentBuffer { samples: result }
    }
}
