#![allow(dead_code)]
use crate::checking_fns::*;
use crate::composite_fns::*;
use crate::replacing_fns::*;

use crate::music_constants::*;

pub fn get_solosequence(ss_title: &str) -> SoloSequence {
    let mut solosequences: Vec<SoloSequence> = vec![
        SoloSequence {
            title: "TestingFunctions",
            instrument: 74, //74 = FLUTE
            velocity: 100,
            abstract_notes: vec![
                -1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 5, 5, 5, 5, 6, 7, 7, 7, 7, 9, 9,
                9, 9, 9, 10, 10, 10, 10, 10, 11,
            ],
            octaves: vec![2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4],
            // abstract_notes: vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            // octaves: vec![3, 3, 3, 3, 4, 4, 4, 4],
            figures: vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 5, 6],
            iter: 5,
            interval_time: SIXTEENTH as u64, // 60 = 1/32
            bpm: 108,
            check_n_replace: vec![
                (trill_on_interval(vec![2], vec![SIXTEENTH])),
                // (trill_on_interval(vec![-2], vec![SIXTEENTH])),
                (trill_on_interval(vec![1], vec![SIXTEENTH])),
                // (trill_on_interval(vec![-1], vec![SIXTEENTH])),
                (approachgliss_on_interval(vec![8], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![9], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![10], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![11], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-8], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-9], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-10], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-11], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![6], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-6], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![5], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-5], vec![SIXTEENTH, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![7], vec![SIXTEENTH, 6, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![4], vec![SIXTEENTH, 3, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![-7], vec![SIXTEENTH, -4, SIXTY_FOURTH])),
                (gracenote_on_interval(vec![-3], vec![SIXTEENTH, -1, SIXTY_FOURTH])),
                (gracenote_on_interval(vec![-2], vec![SIXTEENTH, -1, THIRTY_SECOND])),
                (group_on_interval_in_range(vec![0, 1, 90], vec![SIXTEENTH, 1])),
                (gracenote_on_interval(vec![0], vec![SIXTEENTH, -1, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![0], vec![SIXTEENTH, -2, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![0], vec![SIXTEENTH, -3, SIXTY_FOURTH / 2])),
            ],
        },
        SoloSequence {
            title: "TheAloneLocrio",
            instrument: 75, //74 = FLUTE
            velocity: 100,
            abstract_notes: vec![
                -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 3, 3, 3, 3, 3, 3, 4, 5,
                5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 8, 8, 8, 8, 9, 10, 10, 10, 10, 10, 10, 11,
            ],
            octaves: vec![
                3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5,
            ],
            figures: vec![1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 4, 5, 6, 7],
            iter: 500,
            interval_time: EIGHTH as u64, // 60 = 1/32
            bpm: 90,
            check_n_replace: vec![
                (
                    (Box::new(check_interval_in_range()), vec![-5, 65, 94]),
                    (Box::new(replace_group()), vec![EIGHTH, 2]),
                ),
                (
                    (Box::new(check_interval_in_range()), vec![0, 61, 95]),
                    (Box::new(replace_group()), vec![EIGHTH, 1]),
                ),
                // (
                //     (Box::new(check_interval_in_range()), vec![0,61,95]),
                //        (Box::new(replace_group()), vec![60,1]),
                // ),
                (gliss_on_interval(vec![3], vec![EIGHTH, 1])),
                (gliss_on_interval(vec![-3], vec![EIGHTH, -1])), //
                // (replace_on_interval(vec![-3], vec![60,-1])), //
                (
                    (Box::new(check_interval()), vec![2]),
                    (Box::new(replace_trill()), vec![EIGHTH]),
                ),
                (
                    (Box::new(check_interval()), vec![-2]),
                    (Box::new(replace_trill()), vec![EIGHTH]),
                ),
                (
                    (Box::new(check_interval()), vec![4]),
                    (Box::new(replace_trill()), vec![EIGHTH]),
                ),
                (
                    (Box::new(check_interval()), vec![7]),
                    (Box::new(replace_trill()), vec![EIGHTH]),
                ),
                (
                    (Box::new(check_interval()), vec![5]),
                    (Box::new(replace_trill()), vec![EIGHTH]),
                ),
                (
                    (Box::new(check_interval_in_range()), vec![0, 61, 95]),
                    (Box::new(replace_group()), vec![EIGHTH, 1]),
                ),
                // (
                //     (
                //         // check function
                //         Box::new(
                //             |args: Vec<i64>, a_pitch: i8, _a_dur: u64, b_pitch: i8, _b_dur: u64| {
                //                 if a_pitch == -1 || b_pitch == -1 {
                //                     return false;
                //                 }
                //                 b_pitch - a_pitch == args[0] as i8 //interval
                //             },
                //         ),
                //         vec![4], // args for checking
                //     ),
                //     (
                //         // replace function
                //         Box::new(
                //             |_args: Vec<i64>, _a_pitch: i8, _a_dur: u64, _b_pitch: i8, _b_dur: u64| {
                //                 (vec![], vec![])
                //             },
                //         ),
                //         vec![60], // args for replacing
                //     ),
                //     ),
            ],
        },
        SoloSequence {
            title: "TheLittleLydio",
            instrument: 74,
            velocity: 100,
            abstract_notes: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 6, 6, 6, 6, 6,
                7, 7, 7, 7, 7, 7, 7, 7, 8, 9, 10, 11, 11, 11, 11, 11, 11, -1, -1,
            ],
            octaves: vec![3, 4, 5],
            figures: vec![],
            iter: 1000,
            interval_time: 60, // 1/32
            bpm: 90,
            check_n_replace: vec![],
        },
    ]; // end of Vec of SoloSequences
       // solosequences.iter().map(|ss| {
       //     if ss.name == ss_name {
       //         return ss;
       //     }
       // });
    solosequences.retain(|ss| ss.title == ss_title);
    solosequences.remove(0)
}

pub struct SoloSequence<'a> {
    pub title: &'a str,
    pub instrument: u8,
    pub velocity: u8,
    pub abstract_notes: Vec<i32>,
    pub octaves: Vec<i32>,
    pub figures: Vec<u32>,
    pub iter: u32,
    pub interval_time: u64,
    pub bpm: u32,
    pub check_n_replace: Vec<(
        // a vector filled by tuples with (a (check closure + args) and a (replace closure + args) )
        (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
        (
            Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
            Vec<i64>,
        ),
    )>,
}
