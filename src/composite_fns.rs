use crate::checking_fns::*;
use crate::replacing_fns::*;
use crate::music_constants::*;

/// # Arguments in Vectors
/// Check Args: interval, lower_limit, upper_limit, interval_time -------
/// Replace Args: interval_time, num_repetitions (in a interval_time),  
/// staccato_dur (1 to 12, 1=staccatissimo, 12=legato), 
/// interval[0], interval[1], interval[2] etc...
pub fn tremolopattern_on_interval_in_range(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval_in_range_and_duration()), check_args),
        (Box::new(replace_tremolopattern()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval, interval_time -------
/// Replace Args: interval_time, num_repetitions (in a interval_time), 
/// staccato_dur (1 to 12, 1=staccatissimo, 12=legato)
pub fn tremolo_on_interval(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval_and_duration()), check_args),
        (Box::new(replace_tremolo()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval -------
/// Replace Args: interval_time, difference (interval)
pub fn gliss_on_interval(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval()), check_args),
        (Box::new(replace_gliss()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval, lower_limit, upper_limit, interval_time -------
/// Replace Args: interval_time, initial_direction (interval)
pub fn group_on_interval_in_range(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval_in_range_and_duration()), check_args),
        (Box::new(replace_group()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval, interval_time -------
/// Replace Args: interval_time
pub fn trill_on_interval(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval_and_duration()), check_args),
        (Box::new(replace_trill()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval, interval_time -------
/// Replace Args: glissnote_interval (from the 1st note), gracenote_duration
pub fn gracenote_on_interval(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval_and_duration()), check_args),
        (Box::new(replace_gracenote()), replace_args),
    )
}
/// # Arguments in Vectors
/// Check Args: interval, interval_time -------
/// Replace Args: glissnote_duration
pub fn approachgliss_on_interval(
    check_args: Vec<i32>,
    replace_args: Vec<i32>,
) -> (
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
) {
    (
        (Box::new(check_interval_and_duration()), check_args),
        (Box::new(replace_approachgliss()), replace_args),
    )
}

pub fn tremolo_style() -> Vec<(
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
)>{
    vec![(tremolo_on_interval(vec![MAJ_2, SIXTEENTH],vec![SIXTEENTH,4, 6 ])),
                (tremolo_on_interval(vec![-MAJ_2, SIXTEENTH],vec![SIXTEENTH,4, 6 ])),
                (tremolo_on_interval(vec![MAJ_3, SIXTEENTH],vec![SIXTEENTH,3, 3 ])), // triplet tremolo
                (tremolo_on_interval(vec![-MAJ_3, SIXTEENTH],vec![SIXTEENTH,3, 3 ])), 
                (tremolo_on_interval(vec![MIN_3, SIXTEENTH],vec![SIXTEENTH,3, 1 ])), // very staccato triplet tremolo
                (tremolo_on_interval(vec![-MIN_3, SIXTEENTH],vec![SIXTEENTH,3, 1 ])),  
                (tremolo_on_interval(vec![P_5, SIXTEENTH],vec![SIXTEENTH,2, 9 ])),
                (tremolo_on_interval(vec![-P_5, SIXTEENTH],vec![SIXTEENTH,2, 9 ])),
                (tremolo_on_interval(vec![P_4, SIXTEENTH],vec![SIXTEENTH,2, 2 ])),
                (tremolo_on_interval(vec![-P_4, SIXTEENTH],vec![SIXTEENTH,2, 2 ])),]

}

pub fn concat_c_n_r(
    vector: Vec<Vec<
    (
        (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
        (
            Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
            Vec<i32>,
        ),
    )>>
) -> Vec<
    (
        (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
        (
            Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
            Vec<i32>,
        ),
    )
>{
    let mut finalvector = vec![];
    
    let mut vector = vector;
    let length = vector.len();
    for _ in 0..length {
        let mut subvector = vector.remove(0); 
        let sublength = subvector.len();
        for _ in 0..sublength {
            finalvector.push(subvector.remove(0));
        }
    }
    finalvector
}
// pub fn first_takesall_on_interval(
//     check_args: Vec<i32>,
//     replace_args: Vec<i32>,
// ) -> (
//     (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
//     (
//         Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
//         Vec<i32>,
//     ),
// ) {
//     (
//         (Box::new(check_interval()), check_args),
//         (Box::new(replace_1st_takesall()), replace_args),
//     )
// }
