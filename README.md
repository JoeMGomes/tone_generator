# tone_generator
Small Rust application that writes Pulse Code Modulation ( PCM ) audio from json files.

## Usage

```
USAGE:
    tone_generator [FLAGS] [OPTIONS]

FLAGS:
    -c, --csv        Outputs file as csv
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input JSON File
    -o, --output <output>    Output .bin file name

```
Running tone_generator without arguments will use de default input and output file names ( "input.json" and "output.bin").  
It is recomended to always use the -i and -o options to prevent overwriting existing files.

Run tone_generator to create your PCM file.  
To play your audio file you can use ffplay like shown:
```

 ffplay -showmode 1 -f f32le -ar 48000 output.bin

```
**Note:** the 48000 number above corresponds to the SAMPLE_RATE specified in the input json.

## JSON Format
The programs expects a JSON file with the following format:

```
 {
    "BPM": 120.0,
    "SAMPLE_RATE": 48000.0,
    "PITCH_STANDARD": 440.0,
    "track": [
        {
            "notes": ["B4", "G#3"],
            "duration": 1
        },
        {
            "notes": ["E4"],
            "duration": 1
        },

    ]
}
```
- BPM - Beats per minute (higher BPM means faster music)
- SAMPLE_RATE - Number of samples per second
- PITCH_STANDARD - Frequency in Hertz of the A4 note
- track - Array of "chord" objects
  - chord - Object that stores an array of notes and their duration (in beats)

All notes from C3 to F5 are currently supported. All notes in the 4th octave (? please correct me if my terminology is wrong) ( like A4, C#4, etc) can ommit the number 4 ( C#4 is equal to C#)

## Snippets
The song_snippets folder contains a bit of the song Stairway To Heaven by Led Zeppelin for demonstrations purposes.  
If you make any other songs feel free to make a Pull Request :) 

## v0.1.0
- Can now read json input files and will output PCM audio.
- Added Command Line Interface
  - -i - specify an input json
  - -o - specify an output json
  - -c - output the file as Comma Seperated Values (CSV - helps visualize sound waves) 

## v0.0.1
Can harcode tones and create chords with varied duration.

## Inspiration
This project was inspired by Tsoding's Haskell Music youtube video. Link to his repository [here](https://github.com/tsoding/haskell-music).

## Disclaimer
I am in no way a Rust expert and this project was my first "real" work with the language. Little care was taken to write in "the Rust way".

