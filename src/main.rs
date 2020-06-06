use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};

mod types;
pub use crate::types::*;

mod pulse_math;
pub use crate::pulse_math::*;

fn main() {

    let now = Instant::now();

    let  c: Pulse      = note(-9.0,  1.0);
    let  c_sus : Pulse = note(-8.0,  1.0);
    let  d: Pulse      = note(-7.0,  1.0);
    let  d_sus: Pulse  = note(-6.0,  1.0);
    let  e: Pulse      = note(-5.0,  1.0);
    let  f : Pulse     = note(-4.0,  1.0);
    let  f_sus : Pulse = note(-3.0,  1.0);
    let  g : Pulse     = note(-2.0,  1.0);
    let  g_sus : Pulse = note(-1.0,  1.0);
    let  a: Pulse      = note( 0.0,  1.0);
    let  a_sus: Pulse  = note( 1.0,  1.0);
    let  b: Pulse      = note( 2.0,  1.0);
    let  d_high: Pulse = note( 3.0,  1.0);

    let temp_d = combine_pulse(&a,&c_sus);
    let d_major = combine_pulse(&temp_d, &e);
    let temp_a = combine_pulse(&a,&d);
    let a_major = combine_pulse(&temp_a, &f_sus);

    let mut output = Vec::new();
    output.extend(c.clone());
    // output.extend(c_sus.clone());
    output.extend(d.clone());
    // output.extend(d_sus.clone());
    output.extend(e.clone());
    output.extend(f.clone());
    // output.extend(f_sus.clone());
    output.extend(g.clone());
    // output.extend(g_sus.clone());
    output.extend(a.clone());
    // output.extend(a_sus.clone());
    output.extend(b.clone());
    output.extend(d_high.clone());

    println!("RunTime: {}", now.elapsed().as_millis());

    let write_r = write_to_file(&output);
    match write_r {
        Ok(()) => println!("RunTime: {}", now.elapsed().as_millis()),
        Err(e) => println!("Exportin to bin: {:?}", e),
    }
    // let r = write_csv(&output);
    // match r {
    //     Ok(()) => (),
    //     Err(e) => println!("Error exporting to csv: {:?}", e),
    // }
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
