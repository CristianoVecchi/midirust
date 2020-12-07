use crate::checking_fns::*;
use crate::replacing_fns::*;
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
/**
 * check args: interval, lower limit, upper limit
 * replace args: interval_time, first interval note
 */
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
/**
 *check args: interval
 *replace args: interval_time
 */
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
/**
*check args: interval
*replace args: interval time, grace note interval from the first note, grace note interval time
*/
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
/**
*check args: interval
*replace args: interval time, gliss note interval time
*/
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
