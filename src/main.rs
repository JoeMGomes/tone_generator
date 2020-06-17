use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
extern crate byteorder;
extern crate clap;
extern crate json;

use byteorder::{LittleEndian, WriteBytesExt};
use clap::{App, Arg};

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

    let matches = App::new("Tone Generator")
        .version("0.1.0")
        .author("José Gomes <zemiguelgoems99@gmail.com>")
        .about("Simple JSON \"Piano-like\" music maker.")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Input JSON File"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output .bin file name"),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap_or("input.json");

    let _outname = matches.value_of("file").unwrap_or("output.bin");


    match parse_file(filename, &mut track) {
        Ok(_e) => (),
        Err(e) => panic!("Failed on write: {}", e),
    };

    println!("RunTime: {}", now.elapsed().as_millis());

    let write_r = write_to_file(&track);
    match write_r {
        Ok(()) => println!("RunTime: {}", now.elapsed().as_millis()),
        Err(e) => println!("Exportin to bin: {:?}", e),
    }
    // let r = _write_csv(&track);
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
