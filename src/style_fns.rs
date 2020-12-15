#![allow(unused_imports)]
use crate::checking_fns::*;
use crate::replacing_fns::*;
use crate::composite_fns::*;
use crate::music_constants::*;

/// # Arguments 
/// interval_time 
pub fn tremolo_style(interval_time: i32) -> Vec<(
    (Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> bool>, Vec<i32>),
    (
        Box<dyn Fn(Vec<i32>, i8, i32, i8, i32) -> (Vec<i8>, Vec<i32>)>,
        Vec<i32>,
    ),
)>{
    vec![(tremolo_on_interval(vec![MAJ_2, interval_time],vec![interval_time,4, 6 ])),
                (tremolo_on_interval(vec![-MAJ_2, interval_time],vec![interval_time,4, 6 ])),
                (tremolo_on_interval(vec![MAJ_3, interval_time],vec![interval_time,3, 3 ])), // triplet tremolo
                (tremolo_on_interval(vec![-MAJ_3, interval_time],vec![interval_time,3, 3 ])), 
                (tremolo_on_interval(vec![MIN_3, interval_time],vec![interval_time,3, 1 ])), // very staccato triplet tremolo
                (tremolo_on_interval(vec![-MIN_3, interval_time],vec![interval_time,3, 1 ])),  
                (tremolo_on_interval(vec![P_5, interval_time],vec![interval_time,2, 9 ])),
                (tremolo_on_interval(vec![-P_5, interval_time],vec![interval_time,2, 9 ])),
                (tremolo_on_interval(vec![P_4, interval_time],vec![interval_time,2, 2 ])),
                (tremolo_on_interval(vec![-P_4, interval_time],vec![interval_time,2, 2 ])),]

}

/// # Arguments
/// a Vector of Vectors of 'check and replace' functions (2 tuples in 1 tuple), 
/// can be filled with composite functions or style functions, returns a flat Vector
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