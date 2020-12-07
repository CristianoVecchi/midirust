use crate::music_constants::*;
use crate::solosequences::*;
pub struct MultiSequence<'a> {
    pub title: &'a str,
    pub transpose: Vec<i8>,
    pub instruments: Vec<u8>,
    pub velocities: Vec<u8>,
    pub interval_time: u64,
    pub bpm: u32,
    pub solosequences: Vec<SoloSequence<'a>>,
}
pub fn get_multisequence(ms_title: &str) -> MultiSequence {
    let mut multisequences: Vec<MultiSequence> = vec![MultiSequence {
        title: "MultiSequenceTEST",
        transpose: vec![12, 0, -12, 0, 0, -12],
        instruments: vec![0, 74, 74, 74, 74, 74],
        velocities: vec![75, 110, 105, 100, 95, 90],
        interval_time: SIXTEENTH as u64,
        bpm: 108,
        solosequences: vec![
            get_solosequence("TestingFunctions"),
            get_solosequence("TestingFunctions"),
            get_solosequence("TestingFunctions"),
            // get_solosequence("TestingFunctions"),
            // get_solosequence("TestingFunctions"),
            // get_solosequence("TestingFunctions"),
        ],
    }];
    multisequences.retain(|ms| ms.title == ms_title);
    multisequences.remove(0)
}
