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
struct Drumset {
    play_duration: Duration
}

impl Drumset {
    pub fn new() -> Self {
        Self {
            play_duration: Duration::from_secs_f32(0.3),
        }
    }

    pub fn generate_white_noise(&self, buffer: &mut Vec<f32>, intensity: f32) {
        let mut rng = rand::thread_rng();

        for sample in buffer.iter_mut() {
            *sample = rng.gen_range(-1.0..1.0) * intensity;
        }
    }

    pub fn apply_tone(&self, buffer: &mut Vec<f32>, tone: DrumsetAction) {
        todo!();
    }

    fn mix_buffers(mut a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
        for i in 0..a.len() {
            a[i] += b[i];
        }

        return a;
    }
}

impl Instrument for Drumset {
    type ConcreteValue = DrumsetAction;

    fn render_buffer(&self, buffer_info: BufferInfo, tones: &Tone<Self::ConcreteValue>) -> InstrumentBuffer {
        let target_samples = (self.play_duration.as_secs_f32() * buffer_info.sample_rate as f32)
            .ceil() as usize;

        let intensity = tones.intensity.start * tones.beat_emphasis.unwrap_or(1.0);

        let mut result = vec![0_f32; target_samples];
        
        for tone in &tones.concrete_values {
            let mut buffer = vec![0_f32; target_samples];
            self.generate_white_noise(&mut buffer, intensity);
            self.apply_tone(&mut buffer, *tone);

            result = Self::mix_buffers(result, buffer);
        }


        InstrumentBuffer { samples: result }
    }
}
