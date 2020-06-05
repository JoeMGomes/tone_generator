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

const SAMPLE_RATE: Samples = 48000.0;

fn main() {
    let  mut hz: Hz = 440.0;
    let duration: Seconds = 1.5;

    let now = Instant::now();
    let outputFirst: &mut Pulse = &mut Vec::new();
    freq(hz, duration, outputFirst);

    let outputSecond: &mut Pulse = &mut Vec::new();
    hz = 540.0;
    freq(hz, duration, outputSecond);

    outputFirst.append(outputSecond);

    println!("RunTime: {}", now.elapsed().as_millis());

    let write_r = write_to_file(outputFirst);
    match write_r {
        Ok(()) => println!("RunTime: {}", now.elapsed().as_millis()),
        Err(e) => println!("error parsing header: {:?}", e),
    }
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
