/// # Arguments in Vector
/// interval

pub fn check_interval() -> impl Fn(Vec<i32>, i8, i32, i8, i32)  -> bool {
    |args: Vec<i32>, a_pitch: i8, _a_dur: i32, b_pitch: i8, _b_dur: i32| {
        if a_pitch == -1 || b_pitch == -1 {
            return false;
        }
        b_pitch - a_pitch == args[0] as i8 //interval
    }
}

/// # Arguments in Vector
/// interval, lower_limit, upper_limit
pub fn check_interval_in_range() -> impl Fn(Vec<i32>, i8, i32, i8, i32) -> bool {
    |args: Vec<i32>, a_pitch: i8, _a_dur: i32, b_pitch: i8, _b_dur: i32| {
        if a_pitch == -1 || b_pitch == -1 {
            return false;
        }
        let lower_limit = args[1] as i8;
        let upper_limit = args[2] as i8;
        if a_pitch < lower_limit
            || a_pitch > upper_limit
            || b_pitch < lower_limit
            || b_pitch > upper_limit
        {
            return false;
        }
        b_pitch - a_pitch == args[0] as i8 //interval
    }
}
/// # Arguments in Vector
/// interval, interval_time
pub fn check_interval_and_duration() -> impl Fn(Vec<i32>, i8, i32, i8, i32) -> bool {
    |args: Vec<i32>, a_pitch: i8, a_dur: i32, b_pitch: i8, _b_dur: i32| {
        let interval_time = args[1] ;
        if a_dur < interval_time || a_pitch == -1 || b_pitch == -1 {
            return false;
        }
        b_pitch - a_pitch == args[0] as i8 //interval
    }
}

/// # Arguments in Vector
/// interval, lower_limit, upper_limit, interval_time
pub fn check_interval_in_range_and_duration() -> impl Fn(Vec<i32>, i8, i32, i8, i32) -> bool {
    |args: Vec<i32>, a_pitch: i8, a_dur: i32, b_pitch: i8, _b_dur: i32| {
        let interval_time = args[3];
        if a_dur < interval_time || a_pitch == -1 || b_pitch == -1 {
            return false;
        }
        let lower_limit = args[1] as i8;
        let upper_limit = args[2] as i8;
        if a_pitch < lower_limit
            || a_pitch > upper_limit
            || b_pitch < lower_limit
            || b_pitch > upper_limit
        {
            return false;
        }
        b_pitch - a_pitch == args[0] as i8 //interval
    }
}
