#![allow(dead_code)]
use crate::checking_fns::*;
use crate::composite_fns::*;
use crate::replacing_fns::*;

use crate::music_constants::*;

pub fn get_solosequence(ss_title: &str) -> SoloSequence {
    let mut solosequences: Vec<SoloSequence> = vec![
        SoloSequence {
            title: "TremoloTEST",
            instrument: 65, //PIANO // 74 = RECORDER // 65 = ALTO SAX 
            velocity: 100,
            abstract_notes: vec![
                REST, UNISON, UNISON, UNISON, UNISON,MAJ_2, MAJ_2, MAJ_2, MAJ_2, MAJ_2, 
                MIN_3, MIN_3,MIN_3,MIN_3,MIN_3,MAJ_3, P_4, P_4, P_4, P_4, 
                AUM_4, P_5, P_5, P_5, P_5, MAJ_6, MAJ_6, MAJ_6, MAJ_6, 
                MIN_7, MIN_7, MIN_7, MIN_7, MIN_7, MAJ_7
                //the same scale in halftone numbers:
                // -1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 
                // 4, 5, 5, 5, 5, 6, 7, 7, 7, 7, 
                // 9, 9, 9, 9, 10, 10, 10, 10, 10, 11,
            ],
            octaves: vec![2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4],
            figures: vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 5, 6],
            iter: 700,
            interval_time: SIXTEENTH, // 60 = 1/32
            bpm: 78,
            check_n_replace: vec![
                (tremolo_on_interval(vec![MAJ_2, SIXTEENTH],vec![SIXTEENTH,4, 6 ])),
                (tremolo_on_interval(vec![-MAJ_2, SIXTEENTH],vec![SIXTEENTH,4, 6 ])),
                (tremolo_on_interval(vec![MAJ_3, SIXTEENTH],vec![SIXTEENTH,3, 3 ])), // triplet tremolo
                (tremolo_on_interval(vec![-MAJ_3, SIXTEENTH],vec![SIXTEENTH,3, 3 ])), 
                (tremolo_on_interval(vec![MIN_3, SIXTEENTH],vec![SIXTEENTH,3, 1 ])), // very staccato triplet tremolo
                (tremolo_on_interval(vec![-MIN_3, SIXTEENTH],vec![SIXTEENTH,3, 1 ])),  
                (tremolo_on_interval(vec![P_5, SIXTEENTH],vec![SIXTEENTH,2, 9 ])),
                (tremolo_on_interval(vec![-P_5, SIXTEENTH],vec![SIXTEENTH,2, 9 ])),
                (tremolo_on_interval(vec![P_4, SIXTEENTH],vec![SIXTEENTH,2, 2 ])),
                (tremolo_on_interval(vec![-P_4, SIXTEENTH],vec![SIXTEENTH,2, 2 ])),
                (tremolopattern_on_interval_in_range(vec![MIN_2, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,6, 12, UNISON, -MIN_2, -MAJ_2, -MIN_3, -MAJ_2,-MIN_2])),         
                (tremolopattern_on_interval_in_range(vec![-MIN_2, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,6, 12, UNISON, MIN_2, MAJ_2, MIN_3, MAJ_2,MIN_2])),   
                (tremolopattern_on_interval_in_range(vec![MAJ_6, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,3, 2, P_5, MAJ_3, UNISON ])),         
                (tremolopattern_on_interval_in_range(vec![-MAJ_6, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,4, 6, UNISON, -MIN_3, -P_4,-P_5])), 
                (tremolopattern_on_interval_in_range(vec![MIN_6, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,3, 2, P_5, MIN_3, UNISON ])),         
                (tremolopattern_on_interval_in_range(vec![-MIN_6, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,4, 6, UNISON, -MAJ_3, -AUM_4,-P_5])), 
                (tremolopattern_on_interval_in_range(vec![MIN_7, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,4, 4, UNISON, MIN_3, P_4, AUM_4 ])),         
                (tremolopattern_on_interval_in_range(vec![-MIN_7, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,3, 9, UNISON, -MIN_3, -P_5])), 
                (tremolopattern_on_interval_in_range(vec![MAJ_7, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,4, 12, UNISON, MAJ_3, P_4, P_5 ])),         
                (tremolopattern_on_interval_in_range(vec![-MAJ_7, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,3, 9, UNISON, -MAJ_3, -MIN_6])), 
                (tremolopattern_on_interval_in_range(vec![MAJ_9, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,6, 6, UNISON, MIN_3, P_4, AUM_4,P_5 , MIN_7])),         
                (tremolopattern_on_interval_in_range(vec![-MAJ_9, 1,90,SIXTEENTH],
                    vec![SIXTEENTH,6, 12, UNISON, -MAJ_3,-AUM_4, -MIN_6,-MAJ_7])), 
            ],
        },
        SoloSequence {
            title: "PastoralSolo",
            instrument: 74, //74 = RECORDER
            velocity: 100,
            abstract_notes: vec![
                REST, UNISON, UNISON, UNISON, UNISON,MAJ_2, MAJ_2, MAJ_2, MAJ_2, MAJ_2, 
                MIN_3, MIN_3,MIN_3,MIN_3,MIN_3,MAJ_3, P_4, P_4, P_4, P_4, 
                AUM_4, P_5, P_5, P_5, P_5, MAJ_6, MAJ_6, MAJ_6, MAJ_6, 
                MIN_7, MIN_7, MIN_7, MIN_7, MIN_7, MAJ_7
                //the same scale in halftone numbers:
                // -1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 
                // 4, 5, 5, 5, 5, 6, 7, 7, 7, 7, 
                // 9, 9, 9, 9, 10, 10, 10, 10, 10, 11,
            ],
            octaves: vec![2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4],
            figures: vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 5, 6],
            iter: 200,
            interval_time: SIXTEENTH, // 60 = 1/32
            bpm: 108,
            check_n_replace: vec![
                (trill_on_interval(vec![MAJ_2, SIXTEENTH], vec![SIXTEENTH])),
                (trill_on_interval(vec![MIN_2, SIXTEENTH], vec![SIXTEENTH])),
                (approachgliss_on_interval(vec![MIN_6, SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![MAJ_6, SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![MIN_7, SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![MAJ_7,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-MIN_6,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-MAJ_6,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-MIN_7,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-MAJ_7,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![AUM_4,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-AUM_4,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![P_4,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (approachgliss_on_interval(vec![-P_4,SIXTEENTH], vec![SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![P_5, SIXTEENTH], vec![AUM_4, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![MAJ_3, SIXTEENTH], vec![MIN_3, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![-P_5,SIXTEENTH], vec![-MAJ_3, SIXTY_FOURTH])),
                (gracenote_on_interval(vec![-MIN_3,SIXTEENTH], vec![-MIN_2, SIXTY_FOURTH])),
                (gracenote_on_interval(vec![-MAJ_2,SIXTEENTH], vec![-MIN_2, THIRTY_SECOND])),
                (group_on_interval_in_range(vec![UNISON, 1, 90,SIXTEENTH], vec![SIXTEENTH, 1])),
                (gracenote_on_interval(vec![UNISON,SIXTEENTH], vec![-MIN_2, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![UNISON,SIXTEENTH], vec![-MAJ_2, SIXTY_FOURTH / 2])),
                (gracenote_on_interval(vec![UNISON,SIXTEENTH], vec![-MIN_3, SIXTY_FOURTH / 2])),
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
            interval_time: EIGHTH, // 60 = 1/32
            bpm: 90,
            check_n_replace: vec![
                (
                    (Box::new(check_interval_in_range_and_duration()), vec![-5, 65, 94, EIGHTH]),
                    (Box::new(replace_group()), vec![EIGHTH, 2]),
                ),
                (
                    (Box::new(check_interval_in_range_and_duration()), vec![0, 61, 95, EIGHTH]),
                    (Box::new(replace_group()), vec![EIGHTH, 1]),
                ),
                
                (gliss_on_interval(vec![3], vec![EIGHTH, 1])),
                (gliss_on_interval(vec![-3], vec![EIGHTH, -1])), //
                
                // avoiding the composite functions
                (
                    (Box::new(check_interval_and_duration()), vec![2, EIGHTH]),
                    (Box::new(replace_trill()), vec![EIGHTH]),
                ),
                
                // Closures could be created ad hoc with Box::new(||{})
                // (
                //     (
                //         // check function
                //         Box::new(
                //             |args: Vec<i32>, a_pitch: i8, _a_dur: u32, b_pitch: i8, _b_dur: u32| {
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
                //             |_args: Vec<i32>, _a_pitch: i8, _a_dur: u32, _b_pitch: i8, _b_dur: u32| {
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
    ]; 
    solosequences.retain(|ss| ss.title == ss_title);
    solosequences.remove(0)
}

pub struct SoloSequence<'a> {
    pub title: &'a str,
    pub instrument: u8,
    pub velocity: u8,
    pub abstract_notes: Vec<i32>,
    pub octaves: Vec<i32>,
    pub figures: Vec<i32>,
    pub iter: i32,
    pub interval_time: i32,
    pub bpm: u32,
    pub check_n_replace: Vec<(
        // a vector filled by tuples with (a (check closure + args) and a (replace closure + args) )
        (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
        (
            Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
            Vec<i32>,
        ),
    )>,
}
