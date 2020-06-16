use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
extern crate byteorder;
extern crate json;

use byteorder::{LittleEndian, WriteBytesExt};

mod types;
pub use crate::types::*;

mod pulse_math;
pub use crate::pulse_math::*;

mod freq_generator;
pub use crate::freq_generator::*;

mod parser;
pub use crate::parser::*;

fn main() {

    let now = Instant::now();

    let mut track = get_pulse(25.0);

    // let  _c: Pulse      = pluck_note(-9.0,  0.5);
    // let  _c_sus : Pulse = pluck_note(-8.0,  0.5);
    // let  _d: Pulse      = pluck_note(-7.0,  0.5);
    // let  _d_sus: Pulse  = pluck_note(-6.0,  0.5);
    // let  _e: Pulse      = pluck_note(-5.0,  0.5);
    // let  _f : Pulse     = pluck_note(-4.0,  0.5);
    // let  _f_sus : Pulse = pluck_note(-3.0,  0.5);
    // let  _g : Pulse     = pluck_note(-2.0,  0.5);
    // let  _g_sus : Pulse = pluck_note(-1.0,  0.5);
    // let  _a: Pulse      = pluck_note( 0.0,  0.5);
    // let  _a_sus: Pulse  = pluck_note( 1.0,  0.5);
    // let  _b: Pulse      = pluck_note( 2.0,  0.5);
    // let  _d_high: Pulse = pluck_note( 3.0,  0.5);

    match parse_file(&mut track){
        Ok(_e) => (),
        Err(e) => panic!("Failed on write: {}",e)
    };

    println!("RunTime: {}", now.elapsed().as_millis());

    let write_r = write_to_file(&track);
    match write_r {
        Ok(()) => println!("RunTime: {}", now.elapsed().as_millis()),
        Err(e) => println!("Exportin to bin: {:?}", e),
    }
    let r = _write_csv(&track);
    match r {
        Ok(()) => (),
        Err(e) => println!("Error exporting to csv: {:?}", e),
    }
}

fn _write_csv(pulse: &Pulse) -> std::io::Result<()> {
    let mut pos = 0;
    let mut buffer = File::create("output.csv")?;


    while pos < pulse.len() {
        let v = pulse[pos].to_string() + ",\n"; 
        let _bytes_written = buffer.write(v.as_bytes())?;
        pos += 1;
    }

    Ok(())
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
