#[allow(dead_code)]
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rimd::{MetaEvent, MidiMessage, SMFBuilder};

pub fn replace_by_closures(
    pitches: &mut Vec<i8>,
    durations: &mut Vec<u64>,
    check_fn: Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> bool>,
    check_args: Vec<i64>,
    replace_fn: Box<dyn Fn(Vec<i64>, i8, u64, i8, u64) -> (Vec<i8>, Vec<u64>)>,
    replace_args: Vec<i64>,
) {
    
    let mut index = 0;
    let mut limit = pitches.len() - 1; // because it ends to the second last note
    while index < limit {
        let ch_args = check_args.clone();
        //println!("index: {}", index);
        let a_pitch = pitches[index];
        let b_pitch = pitches[index + 1];
        let a_dur = durations[index];
        let b_dur = durations[index + 1];
        if check_fn(ch_args, a_pitch, a_dur, b_pitch, b_dur) {
            //println!("a {} {}, b {} {}", a_pitch, a_dur, b_pitch, b_dur);
            let rep_args = replace_args.clone();
            let (new_pitches, new_durs) = replace_fn(rep_args, a_pitch, a_dur, b_pitch, b_dur);
            let new_len = new_pitches.len();
            //println!("new_len {}", new_len);
            pitches.splice(index..index + 1, new_pitches[..].iter().cloned());
            durations.splice(index..index + 1, new_durs[..].iter().cloned());
            limit -= 1;
            limit += new_len;
            // println!("limit {}", limit);
            
            index += new_len;
        // println!("index {}", index);
        } else {
            index += 1;
        }
    }
}
pub fn assign_concrete_pitches(pairs: Vec<[i32; 2]>, lowest_limit: u32) -> Vec<i8> {
    //(0..pairs.len()).map(|i| pairs[i][0]  + pairs[i][1]*12 + lowest_limit).collect::<Vec<u32>>()
    pairs
        .iter()
        .map(|pair| {
            if pair[0] == -1 {
                -1 as i8
            } else {
                (pair[0] + pair[1] * 12 + (lowest_limit as i32)) as i8
            }
        })
        //.map(|i| (i as i8))
        .collect::<Vec<i8>>()
}
pub fn assign_concrete_pitches_transposing(
    pairs: Vec<[i32; 2]>,
    lowest_limit: u32,
    transpose: i8,
) -> Vec<i8> {
    //(0..pairs.len()).map(|i| pairs[i][0]  + pairs[i][1]*12 + down_limit).collect::<Vec<u32>>()
    pairs
        .iter()
        .map(|pair| {
            if pair[0] == -1 {
                -1 as i8
            } else {
                (pair[0] + pair[1] * 12 + (lowest_limit as i32)) as i8 + transpose
            }
        })
        //.map(|i| (i as i8))
        .collect::<Vec<i8>>()
}
pub fn matrix3d_rnd_generator(
    iter: u32,
    xs: &mut Vec<i32>,
    ys: &mut Vec<i32>,
    zs: &mut Vec<u32>,
) -> Vec<[i32; 3]> {
    let mut result = Vec::new();
    let mut x_count = 0;
    let mut y_count = 0;
    let mut z_count = 0;
    let xs_len = xs.len();
    let ys_len = ys.len();
    let zs_len = zs.len();
    for _ in 0..iter {
        if x_count % xs_len == 0 {
            xs.shuffle(&mut thread_rng());
            x_count = 0;
        };
        if y_count % ys_len == 0 {
            ys.shuffle(&mut thread_rng());
            y_count = 0;
        };
        if z_count % zs_len == 0 {
            zs.shuffle(&mut thread_rng());
            z_count = 0;
        };
        result.push([xs[x_count], ys[y_count], zs[z_count] as i32]);
        x_count += 1;
        y_count += 1;
        z_count += 1;
    }
    result
}
pub fn matrix2d_rnd_generator(iter: u32, xs: &mut Vec<i32>, ys: &mut Vec<i32>) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    //let used_xs = (0..xs.len()).map(|i| false).collect::<Vec<bool>>();
    
    let mut x_count = 0;
    let mut y_count = 0;
    let xs_len = xs.len();
    let ys_len = ys.len();
    for _ in 0..iter {
        if x_count % xs_len == 0 {
            xs.shuffle(&mut thread_rng());
            x_count = 0;
        };
        if y_count % ys_len == 0 {
            ys.shuffle(&mut thread_rng());
            y_count = 0;
        };
        result.push([xs[x_count], ys[y_count]]);
        x_count += 1;
        y_count += 1;
    }

    result
}
pub fn add_notes_and_durations(
    builder: &mut SMFBuilder,
    track: usize,
    initial_time: u64,
    pitches: Vec<i8>,
    velocity: u8,
    channel: u8,
    durations: Vec<u64>,
) {
    let mut initial_time = initial_time;
    for (i, p) in pitches.iter().enumerate() {
        if *p == -1 {
            initial_time += durations[i];
            //println!("REST {}", initial_time);
            continue;
        }
        builder.add_midi_abs(
            track,                                             // Track number
            initial_time,                                              // absolute initial_time
            MidiMessage::note_on(*p as u8, velocity, channel), // MIDI_MESSAGE: pitch, velocity, channel
        );
        builder.add_midi_abs(
            track,                                              // Track number
            initial_time + durations[i],                                // absolute initial_time
            MidiMessage::note_off(*p as u8, velocity, channel), // pitch, velocity, channel
        );
        initial_time += durations[i];
        //println!("TIME {}", initial_time);
    }
}

pub fn add_notes(
    builder: &mut SMFBuilder,
    track: usize,
    initial_time: u64,
    pitches: Vec<i8>,
    velocity: u8,
    channel: u8,
    interval_time: u64,
) {
    let mut initial_time = initial_time;
    for p in pitches {
        if p == -1 {
            initial_time += interval_time;
            //println!("REST {}", initial_time);
            continue;
        }
        builder.add_midi_abs(
            track,                                            // Track number
            initial_time,                                             // absolute initial_time
            MidiMessage::note_on(p as u8, velocity, channel), // MIDI_MESSAGE: pitch, velocity, channel
        );
        builder.add_midi_abs(
            track,                                             // Track number
            initial_time + interval_time,                              // absolute initial_time
            MidiMessage::note_off(p as u8, velocity, channel), // pitch, velocity, channel
        );
        initial_time += interval_time;
        //println!("TIME {}", initial_time);
    }
}

// bpm = microseconds/quarter note (u64)
// time_signature: data 0 + 1 = x/y (2*pow(y)), data 2 = metronome clicks, data 3 = n. of 32nd notes in a quarter note
// numerator: u8, denominator: u8, clocks_per_tick: u8(DEFAULT=96), num_32nd_notes_per_24_clocks: u8(DEFAULT=8)
// denominator 1=2, 2=4, 3=8, 4=16, 5=32, 6=64
// 96 MIDICLOCKS is the standard QUARTER NOTE
// key signature: i8 -> number of sharps(+)/flats(-), i8 -> 0 = Major | 1 = minor | _ = Invalid
pub fn set_header(
    builder: &mut SMFBuilder,
    initial_time: u64,
    copyright: Option<String>,
    sequence_title: Option<String>,
    instrument_names: Option<Vec<String>>,
    bpm: Option<u32>,
    time_signature: Option<[u8; 4]>,
    key_signature: Option<[u8; 2]>,
) {
    match copyright {
        Some(cop) => builder.add_meta_abs(0, initial_time, MetaEvent::copyright_notice(cop.clone())),
        None => (),
    }
    match sequence_title {
        Some(name) => {
            builder.add_meta_abs(0, initial_time, MetaEvent::sequence_or_track_name(name.clone()))
        }
        None => (),
    }
    match instrument_names {
        Some(instrument_names) => {
            for (i, name) in instrument_names.iter().enumerate() {
                if i == builder.num_tracks() {
                    break;
                };
                builder.add_meta_abs(i, initial_time, MetaEvent::instrument_name(name.clone()))
            }
        }
        None => (),
    }
    match bpm {
        Some(bpm) => builder.add_meta_abs(0, initial_time, MetaEvent::tempo_setting(bpm)),
        None => (),
    }
    match time_signature {
        Some(time_signature) => builder.add_meta_abs(
            0,
            initial_time,
            MetaEvent::time_signature(
                time_signature[0],
                time_signature[1],
                time_signature[2],
                time_signature[3],
            ),
        ),
        None => (),
    }
    match key_signature {
        Some(key_signature) => builder.add_meta_abs(
            0,
            initial_time,
            MetaEvent::key_signature(key_signature[0], key_signature[1]),
        ),
        None => (),
    }
}
