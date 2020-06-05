use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};

type Pulse = Vec<f32>;
type Seconds = f32;
type Samples = f32;
type Hz = f32;
type Semitones = f32;

const SAMPLE_RATE: Samples = 48000.0;
const PITCH_STANDARD : Hz = 440.0;

fn main() {

    let now = Instant::now();

    let mut output_1: Pulse = note(0.0, 1.0);
    let  output_2: Pulse = note(2.0, 1.0);
    let  output_3: Pulse = note(4.0, 1.0);
    let  output_4: Pulse = note(5.0, 1.0);
    let  output_5: Pulse = note(7.0, 1.0);
    let  output_6: Pulse = note(9.0, 1.0);
    let  output_7: Pulse = note(11.0, 1.0);
    let  output_8: Pulse = note(12.0, 1.0);

    output_1.extend(output_2);
    output_1.extend(output_3);
    output_1.extend(output_4);
    output_1.extend(output_5);
    output_1.extend(output_6);
    output_1.extend(output_7);
    output_1.extend(output_8);
    

    println!("RunTime: {}", now.elapsed().as_millis());

    let write_r = write_to_file(&output_1);
    match write_r {
        Ok(()) => println!("RunTime: {}", now.elapsed().as_millis()),
        Err(e) => println!("error parsing header: {:?}", e),
    }
}

fn note(tone: Semitones, duration : Seconds) -> Pulse {
    println!("{:?}", semi_tone(tone));
    return gen_pulse(semi_tone(tone), duration);
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
        let volume = 0.5;
        let sample: f32 = ((i as f32) * step).sin() * volume;

        output_pulse.push(sample);
    }
}

fn write_to_file(data: &Pulse) -> std::io::Result<()> {
    let mut pos = 0;
    let mut buffer = File::create("output.bin")?;

    let mut result: Vec<u8> = Vec::new();
    for &n in data {
        let _ = result.write_f32::<LittleEndian>(n);
    }

    while pos < data.len() {
        let bytes_written = buffer.write(&result[pos..])?;
        pos += bytes_written;
    }

    Ok(())
}
