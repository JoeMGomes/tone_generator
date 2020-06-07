
pub const SAMPLE_RATE: Samples = 48000.0;
pub const PITCH_STANDARD : Hz = 440.0;
pub const BPM : Beats = 120.0;
pub const BEAT_DURATION : Seconds = 60.0/BPM;

pub type Pulse = Vec<f32>;
pub type Seconds = f32;
pub type Samples = f32;
pub type Hz = f32;
pub type Semitones = f32;
pub type Beats = f32;

pub trait Combine {
    fn combine(&self, pulse_one: &impl Combine, pulse_two: &impl Combine);
}

