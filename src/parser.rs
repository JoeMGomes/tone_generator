use std::fs::File;
use std::io;
use std::io::prelude::*;

use json::JsonValue;
use std::str;

use crate::types::*;
use crate::freq_generator::{note};
use crate::pulse_math::{combine_pulse, add_pulse,get_pulse};

pub fn parse_file(output: &mut Pulse) -> io::Result<()> {
    let mut f = File::open("input.json").unwrap();
    let mut content = vec![];
    f.read_to_end(&mut content).unwrap();

    let str_content = str::from_utf8(&content).unwrap();
    let j_con = json::parse(str_content).unwrap();

    println!(
        "BPM: {}
SAMPLE_RATE: {},
PITCH_STANDARD: {},
Track Size: {}",
        j_con["BPM"],
        j_con["SAMPLE_RATE"],
        j_con["PITCH_STANDARD"],
        j_con["track"].len()
    );

    parse_track(output, &j_con["track"]);

    Ok(())
}

pub fn parse_track(output: &mut Pulse, track: &json::JsonValue) {
    let mut c = 0;
    let len = track.len();
    while c < len {
        println!("\nblock: {}", track[c]);
        parse_block(output, &track[c]);
        c += 1;
    }
}

fn parse_block(output: &mut Pulse, block: &JsonValue) {
    // println!("Notes: {}\nDuration: {}", block["notes"], block["duration"]);
    // println!("Nº of: {}", block["notes"][0]);

    
    let duration = block["duration"].as_f32().unwrap();
    let mut chord: Pulse = get_pulse(duration);

    println!("Duration: {}", duration);

    for i in 0..block["notes"].len() {
        let temp = &block["notes"][i];
        let note_str: &str = temp.as_str().unwrap();

        let freq = name_to_freq(note_str);

        let pulse = note(freq, duration);
        chord = combine_pulse(&chord, &pulse);
        println!("Name: {} Semi: {}", note_str, freq);
    }

    add_pulse(output, &chord);
}

fn name_to_freq(name: &str) -> Hz {
    let freq = match name {
        "A" => 0.0,
        "A#" => 1.0,
        "B" => 2.0,
        "C" => 3.0,
        _ => 0.0,
    };

    return freq;
}
