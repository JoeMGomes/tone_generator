use std::fs::File;
use std::io;
use std::io::prelude::*;

use json::JsonValue;
use std::str;

use crate::types::*;
use crate::freq_generator::{note};
use crate::pulse_math::{add_pulse, append_pulse, pluck_pulse};

pub fn parse_file(filename: &str, output: &mut Pulse) -> io::Result<()> {
    let mut f = File::open(filename).unwrap();
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

    let duration = block["duration"].as_f32().unwrap();
    let mut chord: Pulse = Vec::new();

    println!("Duration: {}", duration);

    for i in 0..block["notes"].len() {

        let note_str =  &block["notes"][i].as_str().unwrap();

        let freq = name_to_freq(note_str);

        let pulse = note(freq, duration);
        //first note
        if chord.len() == 0 {
            chord = pulse;
        } else{
            chord = add_pulse(&chord, &pulse);
        }
        println!("Name: {} Semi: {}", note_str, freq);
    }
    pluck_pulse(chord.as_mut());

    append_pulse(output, &chord);
}

fn name_to_freq(name: &str) -> Hz {
    let freq = match name {
        "C3" => -21.0,
        "C#3" => -20.0,
        "D3" => -19.0,
        "D#3" => -18.0,
        "E3" => -17.0,
        "F3" => -16.0,
        "F#3" => -15.0,
        "G3" => -14.0,
        "G#3" => -13.0,
        "A3" => -12.0,
        "A#3" => -11.0,
        "B3" => -10.0,
        "C" | "C4" => -9.0,
        "C#" | "C#4" => -8.0,
        "D" | "D4" => -7.0,
        "D#" | "D#4" => -6.0,
        "E" | "E4" => -5.0,
        "F" | "F4" => -4.0,
        "F#" | "F#4" => -3.0,
        "G" | "G4" => -2.0,
        "G#" | "G#4" => -1.0,
        "A" | "A4" => 0.0,
        "A#" | "A#4" => 1.0,
        "B" | "B4" => 2.0,
        "C5" => 3.0,
        "C#5" => 4.0,
        "D5" => 5.0,
        "D#5" => 6.0,
        "E5" => 7.0,
        "F5" => 8.0,
        _ => -30.0,
    };

    return freq;
}
