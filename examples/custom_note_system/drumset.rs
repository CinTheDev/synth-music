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

    pub fn generate_tone(&self, time: Duration) -> f32 {
        let frequency = 50.0;
        predefined::sine_wave(frequency, time) * self.decay(time)
    }

    pub fn generate_white_noise(&self, buffer: &mut Vec<f32>, intensity: f32) {
        let mut rng = rand::thread_rng();

        for sample in buffer.iter_mut() {
            *sample = rng.gen_range(-1.0..1.0) * intensity;
        }
    }

    pub fn apply_tone(&self, buffer: &mut Vec<f32>, sample_rate: u32, tone: DrumsetAction) {
        // Let's apply simple filter first
        use biquad::*;

        let f_cutoff = 1000.hz();
        let f_sample = sample_rate.hz();

        let coeffs = Coefficients::<f32>::from_params(
            Type::LowPass,
            f_sample,
            f_cutoff,
            Q_BUTTERWORTH_F32,
        ).unwrap();

        let mut biquad = DirectForm1::<f32>::new(coeffs);
        
        for sample in buffer.iter_mut() {
            *sample = biquad.run(*sample);
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

        let intensity = tones.intensity.start * tones.beat_emphasis.unwrap_or(1.0);

        let mut result = vec![0_f32; target_samples];
        
        for _ in &tones.concrete_values {
            for i in 0..target_samples {
                let time = buffer_info.time_from_index(i);
                result[i] = self.generate_tone(time) * intensity;
            }
            //let mut buffer = vec![0_f32; target_samples];
            //self.generate_white_noise(&mut buffer, intensity);
            //self.apply_tone(&mut buffer, buffer_info.sample_rate, *tone);

            //result = Self::mix_buffers(result, buffer);
        }


        InstrumentBuffer { samples: result }
    }
}
