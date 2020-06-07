use crate::types::*;
use crate::freq_generator::*;

pub fn pluck_note(tone: Semitones, beats : Beats) -> Pulse {
    return release(&attack(&note(tone, beats)));
}

pub fn attack(pulse: &Pulse) -> Pulse {
    let duration = (pulse.len() as f32) / SAMPLE_RATE;
    return mul_pulse(pulse, &gen_attack(duration));
}

pub fn gen_attack(duration: Seconds) -> Pulse {
    let sample_number = SAMPLE_RATE * duration;
    let attack_time = sample_number * 0.07;
    let step = 1.0 / attack_time;
    let mut attack: Pulse = Vec::with_capacity(sample_number as usize);
    let mut i = 0.0;
    while i < attack_time {
        attack.push(i * step);
        i += 1.0;
    }
    while i < sample_number {
        attack.push(1.0);
        i += 1.0;
    }
    return attack;
}

pub fn release(pulse: &Pulse) -> Pulse {
    let duration = pulse.len() as f32 / SAMPLE_RATE;
    return mul_pulse(pulse, &gen_release(duration));
}

pub fn gen_release(duration: Seconds) -> Pulse {
    let sample_number = SAMPLE_RATE * duration;
    let release_time = sample_number * 0.5;
    let step = 1.0 / release_time;

    let mut release_vec: Pulse = vec![1.0; sample_number as usize];
    let mut count = sample_number - 1.0;
    let mut i = 0.0;

    while count > sample_number - release_time {
        let r = i * step;
        if r > 1.0 {
            release_vec[count as usize] = 1.0;
        } else {
            release_vec[count as usize] = r;
        }
        i += 1.0;
        count -= 1.0;
    }
    return release_vec;
}

/// element-wise multiplication for vecs
pub fn combine_pulse(v1: &Pulse, v2: &Pulse) -> Pulse {
    if v1.len() != v2.len() {
        panic!("Cannot multiply vectors of different lengths!")
    }

    v1.iter().zip(v2).map(|(&i1, &i2)| i1 + i2).collect()
}

/// element-wise multiplication for vecs
pub fn mul_pulse(v1: &Pulse, v2: &Pulse) -> Pulse {
    if v1.len() != v2.len() {
        panic!("Cannot multiply vectors of different lengths!")
    }

    v1.iter().zip(v2).map(|(&i1, &i2)| i1 * i2).collect()
}

pub fn add_pulse(dest: &mut Pulse, v2: &Pulse) {
    if dest.capacity() - dest.len() < v2.len() {
        panic!(
            "Destination pulse does not have enough space. Free space: {} Tried to use: {}",
            dest.capacity() - dest.len(),
            v2.len()
        );
    }
    dest.extend(v2.clone());
}

pub fn get_pulse(size: Beats) -> Pulse {
    return Vec::with_capacity((size * BEAT_DURATION * SAMPLE_RATE) as usize);
}
