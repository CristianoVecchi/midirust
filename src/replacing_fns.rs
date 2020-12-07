/// # Arguments in Vector
/// interval_time, gracenote_interval, gracenote_duration
pub fn replace_gracenote() -> impl Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>) {
    |args: Vec<i64>, a_pitch: i8, a_dur: u64, _b_pitch: i8, _b_dur: u64| {
        let interval_time = args[0] as u64;
        if a_dur < interval_time {
            (vec![a_pitch], vec![a_dur])
        } else {
            let gracenote_interval = args[1] as i8;
            let gracenote_dur = args[2] as u64;
            (
                vec![a_pitch, a_pitch + gracenote_interval],
                vec![a_dur - gracenote_dur, gracenote_dur],
            )
        }
    }
}
/// # Arguments in Vector
/// interval_time, initial_direction
pub fn replace_group() -> impl Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>) {
    |args: Vec<i64>, a_pitch: i8, a_dur: u64, _b_pitch: i8, _b_dur: u64| {
        let interval_time = args[0] as u64;
        let a_qdur = a_dur / interval_time;
        if a_qdur < 2 || a_dur < interval_time {
            (vec![a_pitch], vec![a_dur])
        } else {
            //println!("REPLACED");
            let group_note_dur = interval_time / 2;
            let initial_direction = args[1] as i8;
            (
                vec![
                    a_pitch,
                    a_pitch + initial_direction,
                    a_pitch,
                    a_pitch - initial_direction,
                ],
                vec![
                    a_dur - (group_note_dur * 3),
                    group_note_dur,
                    group_note_dur,
                    group_note_dur,
                ],
            )
        }
    }
}
/// # Arguments in Vector
/// interval_time, glissnote_duration
pub fn replace_approachgliss() -> impl Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>) {
    |args: Vec<i64>, a_pitch: i8, a_dur: u64, b_pitch: i8, _b_dur: u64| {
        let interval_time = args[0] as u64;
        if a_dur < interval_time {
            (vec![a_pitch], vec![a_dur])
        } else {
            let gliss_note_dur = args[1] as u64;
            let n_pos_notes = a_dur / gliss_note_dur;
            let n_gliss_notes = (a_pitch - b_pitch).abs() as u64;
            let direction = if a_pitch < b_pitch { 1 } else { -1 };
            let mut pitches = vec![];
            let mut durs = vec![];
            if n_pos_notes >= n_gliss_notes {
                pitches.push(a_pitch);
                durs.push(a_dur - (n_gliss_notes - 1) * gliss_note_dur);
                let mut new_pitch = a_pitch;
                for _ in 1..n_gliss_notes {
                    pitches.push(new_pitch);
                    new_pitch += direction;
                    durs.push(gliss_note_dur)
                }
                (pitches, durs)
            } else {
                pitches.push(a_pitch);
                durs.push(gliss_note_dur);
                let mut new_pitch = a_pitch + (direction * (n_gliss_notes - n_pos_notes) as i8);
                for _ in 1..n_pos_notes {
                    pitches.push(new_pitch);
                    new_pitch += direction;
                    durs.push(gliss_note_dur);
                }
                (pitches, durs)
            }
        }
    }
}

/// # Arguments in Vector
/// interval_time
pub fn replace_trill() -> impl Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>) {
    |args: Vec<i64>, a_pitch: i8, a_dur: u64, b_pitch: i8, _b_dur: u64| {
        let interval_time = args[0] as u64;
        let a_qdur = a_dur / interval_time;
        if a_qdur == 1 || a_dur < interval_time {
            (vec![a_pitch], vec![a_dur])
        } else {
            //let num_trill_notes = a_qdur * 2;
            let trill_note_dur = interval_time / 2;
            let mut new_pitches = vec![];
            let mut new_durs = vec![];
            for _ in 0..a_qdur - 1 {
                new_pitches.push(a_pitch);
                new_pitches.push(b_pitch);
                new_durs.push(trill_note_dur);
                new_durs.push(trill_note_dur);
            }
            new_pitches.push(a_pitch);
            new_pitches.push(b_pitch);
            new_pitches.push(a_pitch);
            let triplet_note_dur = interval_time / 3;
            new_durs.push(triplet_note_dur);
            new_durs.push(triplet_note_dur);
            new_durs.push(triplet_note_dur);
            (new_pitches, new_durs)
        }
    }
}

/// # Arguments in Vector
/// interval_time, difference
pub fn replace_gliss() -> impl Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>) {
    |args: Vec<i64>, a_pitch: i8, a_dur: u64, _b_pitch: i8, _b_dur: u64| {
        let interval_time = args[0] as u64;
        let difference = args[1];
        let a_qdur = a_dur / interval_time;
        if a_qdur == 2 {
            let qdur0 = a_qdur - 1;
            let qdur1 = 0.5;
            let qdur2 = 0.5;
            (
                vec![
                    a_pitch,
                    a_pitch + difference as i8,
                    a_pitch + difference as i8 * 2,
                ],
                vec![
                    qdur0 * interval_time,
                    (qdur1 * interval_time as f64) as u64,
                    (qdur2 * interval_time as f64) as u64,
                ],
            )
        } else if a_qdur > 2 {
            let qdur0 = a_qdur - 2;
            let qdur1 = 1;
            let qdur2 = 1;
            (
                vec![
                    a_pitch,
                    a_pitch + difference as i8,
                    a_pitch + difference as i8 * 2,
                ],
                vec![
                    qdur0 * interval_time,
                    qdur1 * interval_time,
                    qdur2 * interval_time,
                ],
            )
        } else {
            (vec![a_pitch], vec![a_dur])
        }
    }
}
// the following function belongs to the initial phase of this project, 
// when the replacing functions operated on the first and second note; 
// to avoid a segmentation into pairs it was decided to replace only the first note; 
// however this type of functions can be added later.
//
// pub fn replace_1st_takesall() -> impl Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>) {
//     |args: Vec<i64>, a_pitch: i8, a_dur: u64, b_pitch: i8, b_dur: u64| {
//         //let interval_time = args[0] as u64;
//         //let a_qdur = a_dur / interval_time;
//         let check_dur = args[1] as u64;
//         if a_dur != check_dur {
//             (vec![a_pitch], vec![a_dur])
//         } else {
//             (vec![a_pitch], vec![a_dur + b_dur])
//         }
//     }
// }