use std::f32::consts::PI;

pub use crate::types::*;

const SAMPLE_RATE: Samples = 48000.0;
const PITCH_STANDARD : Hz = 440.0;
const BPM : Beats = 120.0;
const BEAT_DURATION : Seconds = 60.0/BPM;


pub fn attack_release(pulse : &Pulse)->Pulse{
    return release(&attack(pulse));
}

pub fn attack(pulse : &Pulse)-> Pulse
{
    let duration = (pulse.len() as f32)/SAMPLE_RATE ;
    return mul_pulse(pulse, &gen_attack(duration));
}

pub fn gen_attack(duration: Seconds) ->Pulse{
    let sample_number = SAMPLE_RATE * duration;
    let attack_time = sample_number * 0.07;
    let step = 1.0/attack_time;
    let mut attack: Pulse = Vec::with_capacity(sample_number as usize);
    let mut i = 0.0;
    while i < attack_time {
        attack.push(i*step);
        i+= 1.0;
    }
    while i < sample_number{
        attack.push(1.0);
        i+= 1.0;
    }
    return attack;
}

pub fn release(pulse : &Pulse)-> Pulse
{
    let duration = pulse.len() as f32/SAMPLE_RATE ;
    return mul_pulse(pulse, &gen_release(duration));
}

pub fn gen_release(duration: Seconds) ->Pulse{
    let sample_number = SAMPLE_RATE * duration;
    let release_time = sample_number * 0.5;
    let step = 1.0/release_time;

    let mut release_vec: Pulse = vec![1.0; sample_number as usize];
    let mut count = sample_number - 1.0;
    let mut i = 0.0;

    while count > sample_number-release_time {
        let r = i*step;
        if r > 1.0{
            release_vec[count as usize] = 1.0;
        }else{
            release_vec[count as usize] = r;
        }
        i += 1.0;
        count -= 1.0;
    }
    
    return release_vec;
}


/// element-wise multiplication for vecs
pub fn combine_pulse(v1: &Pulse, v2: &Pulse) -> Pulse
{
    if v1.len() != v2.len() {
        panic!("Cannot multiply vectors of different lengths!")
    }

    v1.iter().zip(v2).map(|(&i1, &i2)| i1 + i2).collect()
}

/// element-wise multiplication for vecs
pub fn mul_pulse(v1: &Pulse, v2: &Pulse) -> Pulse
{
    if v1.len() != v2.len() {
        panic!("Cannot multiply vectors of different lengths!")
    }

    v1.iter().zip(v2).map(|(&i1, &i2)| i1 * i2).collect()
}


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