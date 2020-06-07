
use std::f32::consts::PI;

use crate::types::*;


pub fn note(tone: Semitones, beats : Beats) -> Pulse {
    return gen_pulse(semi_tone(tone), beats * BEAT_DURATION);
}

fn semi_tone( tone : Semitones) -> Hz {
    let x : f32 = 2.0;
    return PITCH_STANDARD * (x.powf(1.0/12.0)).powf(tone);
}

fn gen_pulse(hz: Hz, duration : Seconds) -> Pulse{

    let output: &mut Pulse = &mut Vec::new();
    freq(hz, duration, output);
    return output.to_vec();
}

fn freq(hz: Hz, duration: Seconds, output_pulse: &mut Pulse) {
    let sample_number: usize = (SAMPLE_RATE * duration) as usize;

    for i in 0..sample_number {
        let step = (hz * 2.0 * PI) / SAMPLE_RATE;
        let volume = 0.2;
        let sample: f32 = ((i as f32) * step).sin() * volume;

        output_pulse.push(sample);
    }
}