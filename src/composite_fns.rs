use crate::checking_fns::*;
use crate::replacing_fns::*;

/// # Arguments in Vectors
/// Check Args: interval -------
/// Replace Args: interval_time, difference (interval)
pub fn gliss_on_interval(
    check_args: Vec<i64>,
    replace_args: Vec<i64>,
) -> (
    (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
    (
        Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
        Vec<i64>,
    ),
) {
    (
        (Box::new(check_interval()), check_args),
        (Box::new(replace_gliss()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval, lower_limit, upper_limit
/// Replace Args: interval_time, initial_direction (interval)
pub fn group_on_interval_in_range(
    check_args: Vec<i64>,
    replace_args: Vec<i64>,
) -> (
    (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
    (
        Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
        Vec<i64>,
    ),
) {
    (
        (Box::new(check_interval_in_range()), check_args),
        (Box::new(replace_group()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval -------
/// Replace Args: interval_time
pub fn trill_on_interval(
    check_args: Vec<i64>,
    replace_args: Vec<i64>,
) -> (
    (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
    (
        Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
        Vec<i64>,
    ),
) {
    (
        (Box::new(check_interval()), check_args),
        (Box::new(replace_trill()), replace_args),
    )
}

/// # Arguments in Vectors
/// Check Args: interval -------
/// Replace Args: interval_time, glissnote_interval (from the 1st note), gracenote_duration
pub fn gracenote_on_interval(
    check_args: Vec<i64>,
    replace_args: Vec<i64>,
) -> (
    (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
    (
        Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
        Vec<i64>,
    ),
) {
    (
        (Box::new(check_interval()), check_args),
        (Box::new(replace_gracenote()), replace_args),
    )
}
/// # Arguments in Vectors
/// Check Args: interval -------
/// Replace Args: interval_time, glissnote_duration
pub fn approachgliss_on_interval(
    check_args: Vec<i64>,
    replace_args: Vec<i64>,
) -> (
    (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
    (
        Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
        Vec<i64>,
    ),
) {
    (
        (Box::new(check_interval()), check_args),
        (Box::new(replace_approachgliss()), replace_args),
    )
}
// pub fn first_takesall_on_interval(
//     check_args: Vec<i64>,
//     replace_args: Vec<i64>,
// ) -> (
//     (Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>, Vec<i64>),
//     (
//         Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
//         Vec<i64>,
//     ),
// ) {
//     (
//         (Box::new(check_interval()), check_args),
//         (Box::new(replace_1st_takesall()), replace_args),
//     )
// }
