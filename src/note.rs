use crate::types::*;

pub struct Note {
    tone: Semitones,
    duration: Beats
}

pub fn get_note(tone: Semitones, duration: Beats)->Note{
    Note{
        tone: tone,
        duration: duration
    }
}
