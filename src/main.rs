#![allow(dead_code)]
#[allow(unused_imports)]
use rimd::{
    MidiMessage, SMFBuilder, SMFError, SMFFormat,
    SMFWriter, Track, TrackEvent, SMF,
};
use std::thread;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};


mod replacing_fns;
mod checking_fns;
mod composite_fns;
mod style_fns;
mod music_constants;
mod note_fns; use note_fns::*;
mod solosequences; use solosequences::*;
mod multisequences; use multisequences::*;
use std::path::Path;
use std::time::Instant;

/// WARNING: Change this notice for your own compositions!
const COPYRIGHT_NOTICE: &str = "Copyright (c) 2020 Cristiano Vecchi";
/// the target PATH where the midi file will be created
const RUSTMIDI_TEST_PATH: &str = "/home/cris/Desktop/";
//const RUSTMIDI_TEST_PATH: &str = "C:\\Users\\Cristiano\\Desktop\\"; //Win
const _RUSTMIDI_TEST: &str = "/home/cris/Desktop/RUSTMIDI_TEST.MID";
const _SINGLE_NOTE: &str = "/home/cris/Desktop/singleNote.mid";
const _BACH_BRAND1: &str = "/home/cris/Desktop/BACH_BRAND1.mid";
const _GENIUS1: &str = "/home/cris/Desktop/Genius1.mid";
const _BACH_INVENTIO1: &str = "/home/cris/Desktop/bach_inventio1.mid";
// RUN: Ctrl+Shift+B

/// options: generates a SoloSequence or a MultiSequence 
/// choosing among the available ones in the related files 
/// (solosequences.rs and multisequences.rs)
fn main() {
    let start = Instant::now();
    //let solosequence = "TheAloneLocrio";
    // let solosequence = "TremoloTEST";
    // write_solosequence(get_solosequence(solosequence));
    let multisequence = "Pastoral";
    write_multisequence(get_multisequence(multisequence));
    let elapsed = start.elapsed();
    println!("Midifile created in {} milliseconds", elapsed.as_millis());

    let start2 = Instant::now();
    write_multisequence_parallel(get_multisequence(multisequence));
    let elapsed2 = start2.elapsed();
    println!("Midifile parallel created in {} milliseconds", elapsed2.as_millis());
}
/// 1_000_000 microseconds == 1 second
/// # Arguments
/// 'bpm' - beats per minute, converts the metronome signature to microseconds
fn bpm_to_microseconds(bpm: u32) -> u32 {
    // 1_000_000 : 60 = x : 1/bpm
    (1_000_000 / bpm) * 60
}

/// Creates a MidiFile with Track 0 as Header, Tracks 1toN as instruments on channel N-1.
/// If are present, 
/// some values in the MultiSequence instance 
/// override the SoloSequence ones (intruments, velocities, bpm, title, interval_time,
/// transpositions will not be made if the value is missing).
/// 'const RUSTMIDI_TEST_PATH' is the target directory.
fn write_multisequence_parallel(mseq: MultiSequence) {
    let start_h = Instant::now();
    let n_solosequences = mseq.solosequences.len();
    let mut instrument_names = vec![String::from("Globals")];
    let mut instruments = vec![];
    let mut velocities = vec![];
    let mut transpositions = vec![];
    let mut interval_times = vec![];
    let mut figures_nums = vec![];
    let pitches_vec: Vec<Vec<i8>>= vec![];
    let durs_vec: Vec<Vec<i32>>= vec![];
    let pitches_vec_mutex = Arc::new(Mutex::new(pitches_vec));
    let durs_vec_mutex = Arc::new(Mutex::new(durs_vec));
    let pitches_vec_mutex_clone = Arc::clone(&pitches_vec_mutex);
    let durs_vec_mutex_clone = Arc::clone(&durs_vec_mutex);
    for tr in 0..mseq.solosequences.len(){
       
        let mut pv = pitches_vec_mutex_clone.lock().unwrap();
        pv.push(vec![]);
       
        let mut dv = durs_vec_mutex_clone.lock().unwrap();
        dv.push(vec![]);
        drop(pv);
        drop(dv);
        let sseq = &mseq.solosequences[tr];
        instrument_names.push(String::from(format!("Instr. n. = {}", mseq.solosequences[tr].instrument)));
        if mseq.instruments.len() > tr {
            instruments.push(MidiMessage::program_change(mseq.instruments[tr], tr as u8))//program, channel
        } else {
            instruments.push(MidiMessage::program_change(sseq.instrument, tr as u8)) //program, channel
        };  
        if mseq.velocities.len() > tr {
            velocities.push(mseq.velocities[tr]);
        } else {
            velocities.push(sseq.velocity);
        }; 
        if mseq.transpose.len() > tr {
            transpositions.push(mseq.transpose[tr]);
        } else {
            transpositions.push(sseq.transpose);
        }
        interval_times.push(sseq.interval_time);
        figures_nums.push(sseq.figures.len());
    }
    let elapsed_h = start_h.elapsed();
                println!("Header completed in {} milliseconds",elapsed_h.as_millis()); 
    //let data_mutex = Arc::new(Mutex::new(vec![1, 2, 3, 4]));
    
    
    let mut solosequences = mseq.solosequences;
    let mut children = vec![];

    
    
    for tr in 0..n_solosequences{
        let sseq = solosequences.remove(0);
        let figures_num = figures_nums[tr];
        let interval = interval_times[tr];
        let transpose = transpositions[tr];
        let iter = sseq.iter;
        let mut xs = sseq.abstract_notes;
        let mut ys = sseq.octaves;
        let mut zs = sseq.figures; 
        
        
        let c_n_r_len = sseq.check_n_replace.len();
        let check_n_replace_fns = sseq.check_n_replace;        
        let pitches_vec_mutex_clone = Arc::clone(&pitches_vec_mutex);
        let durs_vec_mutex_clone = Arc::clone(&durs_vec_mutex);
        
        let start = Instant::now();
        println!("Starting Thread #{}",tr);
        children.push(thread::spawn(move || {   
            
            let mut pitches: Vec<i8>;
            let mut durations: Vec<i32>;
            if figures_num == 0 {
                let result = matrix2d_rnd_generator(iter, &mut xs, &mut ys);
                pitches = assign_concrete_pitches_transposing(result, 24, transpose);
                
                let mut pv = pitches_vec_mutex_clone.lock().unwrap();
               
                let _ = std::mem::replace(&mut pv[tr], pitches);
                
                drop(pv);
               
            } else {
                let result = matrix3d_rnd_generator(iter, &mut xs, &mut ys, &mut zs);
                let pairs = result
                    .iter()
                    .map(|array| [array[0], array[1]])
                    .collect::<Vec<[i32; 2]>>();
                durations = result 
                    .iter()
                    .map(|array| array[2] * interval)
                    .collect::<Vec<i32>>();
                pitches = assign_concrete_pitches_transposing(pairs, 24, transpose);
                if c_n_r_len != 0 {
                    for c_n_r in check_n_replace_fns {
                        let check_fn = c_n_r.0.0;
                        let replace_fn = c_n_r.1.0;
                        let check_args = c_n_r.0.1;
                        let replace_args = c_n_r.1.1;
                        replace_by_closures(
                            &mut pitches,
                            &mut durations,
                            check_fn,
                            check_args,
                            replace_fn,
                            replace_args,
                        );
                        
                    }
                }
                let mut pv = pitches_vec_mutex_clone.lock().unwrap();
                let mut dv = durs_vec_mutex_clone.lock().unwrap();
                let _ = std::mem::replace(&mut pv[tr], pitches);
                let _ = std::mem::replace(&mut dv[tr], durations); 
                drop(pv);
                drop(dv); 
                let elapsed = start.elapsed();
                println!("Thread #{} terminated in {} milliseconds", tr, elapsed.as_millis());            
            }
            
        }));
        
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    let mut builder = SMFBuilder::new();
    builder.add_track();
    set_header(
        &mut builder,
        0,
        Some(String::from(COPYRIGHT_NOTICE)),
        Some(String::from(mseq.title)),
        Some(instrument_names),
        Some(bpm_to_microseconds(mseq.bpm)), // micro_second (1_000_000 of a second) in a quarter note (q=60)
        Some([4, 2, 96, 8]),                // 4/4
        Some([0, 0]),                       // C Major
    );

    // let pitches_vec_mutex_clone = Arc::clone(&pitches_vec_mutex);
    // let durs_vec_mutex_clone = Arc::clone(&durs_vec_mutex);
    for tr in 0..n_solosequences {
        builder.add_track();
        let instrument = instruments.remove(0);
        
        let mut pv = pitches_vec_mutex_clone.lock().unwrap();
        
        let mut dv = durs_vec_mutex_clone.lock().unwrap();
        let pitches = pv.remove(0);
        let durations = dv.remove(0);
        drop(pv);
        drop(dv);
        let volume = MidiMessage::control_change(7, 90, tr as u8); // volume, value of volume, channel
          
        builder.add_midi_abs(tr+1, 0, instrument); // track, time, midimessage
        builder.add_midi_abs(tr+1, 0, volume);
        if figures_nums[tr] == 0 {
            add_notes(
                &mut builder,
                1,
                0,
                pitches,
                velocities[tr],
                0,
                interval_times[tr],
            ); // 480 = 1/4, 60 = 1/32
        } else {
            add_notes_and_durations(&mut builder, tr + 1, 0, pitches, velocities[tr], tr as u8, durations);
        }
        
    }
        
    let mut midipiece = builder.result();
    // The unit of time for delta timing. If the value is positive,
    // then it represents the units per beat. For example, +96 would
    // mean 96 ticks per beat. If the value is negative, delta times
    // are in SMPTEpub division: i16, compatible units.
    midipiece.division = 480; // 1 tick = 24 clocks; let rep_args = replace_args.clone();quarter = 480 ticks; 1/32 = 60 ticks
    let writer = SMFWriter::from_smf(midipiece);
    let path = format!("{}{}_par.mid", RUSTMIDI_TEST_PATH, mseq.title);
    writer.write_to_file(&Path::new(&path[..])).unwrap();
    //read_file(&path[..]);
      

}
fn write_multisequence(mseq: MultiSequence) {
    let n_solosequences = mseq.solosequences.len();
    let mut instrument_names = vec![String::from("Globals")];
    let mut instruments = vec![];
    let mut velocities = vec![];
    let mut transpositions = vec![];
    let mut interval_times = vec![];
    let mut figures_nums = vec![];
    let mut pitches_vec = vec![];
    let mut durs_vec: Vec<Vec<i32>>= vec![];
    for tr in 0..mseq.solosequences.len(){
        let sseq = &mseq.solosequences[tr];
        instrument_names.push(String::from(format!("Instr. n. = {}", mseq.solosequences[tr].instrument)));
        if mseq.instruments.len() > tr {
            instruments.push(MidiMessage::program_change(mseq.instruments[tr], tr as u8))//program, channel
        } else {
            instruments.push(MidiMessage::program_change(sseq.instrument, tr as u8)) //program, channel
        };  
        if mseq.velocities.len() > tr {
            velocities.push(mseq.velocities[tr]);
        } else {
            velocities.push(sseq.velocity);
        }; 
        if mseq.transpose.len() > tr {
            transpositions.push(mseq.transpose[tr]);
        } else {
            transpositions.push(sseq.transpose);
        }
        interval_times.push(sseq.interval_time);
        figures_nums.push(sseq.figures.len());
    }
    
    
    let mut solosequences = mseq.solosequences;
    
    for tr in 0..n_solosequences{   
        let sseq = solosequences.remove(0);
        let figures_num = figures_nums[tr];
        let interval = interval_times[tr];
        let transpose = transpositions[tr];
        let iter = sseq.iter;
        let mut xs = sseq.abstract_notes;
        let mut ys = sseq.octaves;
        let mut zs = sseq.figures; 

        let c_n_r_len = sseq.check_n_replace.len();
        let check_n_replace_fns = sseq.check_n_replace;
        if figures_num == 0 {
            let result = matrix2d_rnd_generator(iter, &mut xs, &mut ys);
            let pitches = assign_concrete_pitches_transposing(result, 24, transpose);
            pitches_vec.push(pitches);
            durs_vec.push(vec![]);     
        } else {
            let result = matrix3d_rnd_generator(iter, &mut xs, &mut ys, &mut zs);
            let pairs = result
                .iter()
                .map(|array| [array[0], array[1]])
                .collect::<Vec<[i32; 2]>>();
            let mut durations = result 
                .iter()
                .map(|array| array[2] * interval)
                .collect::<Vec<i32>>();
            let mut pitches = assign_concrete_pitches_transposing(pairs, 24, transpose);
            if c_n_r_len != 0 {
                for c_n_r in check_n_replace_fns {
                    let check_fn = c_n_r.0.0;
                    let replace_fn = c_n_r.1.0;
                    let check_args = c_n_r.0.1;
                    let replace_args = c_n_r.1.1;
                    replace_by_closures(
                        &mut pitches,
                        &mut durations,
                        check_fn,
                        check_args,
                        replace_fn,
                        replace_args,
                    );
                    
                }
            }
            pitches_vec.push(pitches);
            durs_vec.push(durations);
           
            
        }
    }
    let mut builder = SMFBuilder::new();
    builder.add_track();
    set_header(
        &mut builder,
        0,
        Some(String::from(COPYRIGHT_NOTICE)),
        Some(String::from(mseq.title)),
        Some(instrument_names),
        Some(bpm_to_microseconds(mseq.bpm)), // micro_second (1_000_000 of a second) in a quarter note (q=60)
        Some([4, 2, 96, 8]),                // 4/4
        Some([0, 0]),                       // C Major
    );

       
    for tr in 0..n_solosequences {
        builder.add_track();
        let instrument = instruments.remove(0);
        let pitches = pitches_vec.remove(0);
        let durations = durs_vec.remove(0);
        let volume = MidiMessage::control_change(7, 90, tr as u8); // volume, value of volume, channel
          
        builder.add_midi_abs(tr+1, 0, instrument); // track, time, midimessage
        builder.add_midi_abs(tr+1, 0, volume);
        if figures_nums[tr] == 0 {
            add_notes(
                &mut builder,
                1,
                0,
                pitches,
                velocities[tr],
                0,
                interval_times[tr],
            ); // 480 = 1/4, 60 = 1/32
        } else {
            add_notes_and_durations(&mut builder, tr + 1, 0, pitches, velocities[tr], tr as u8, durations);
        }
        
    }
        
    let mut midipiece = builder.result();
    // The unit of time for delta timing. If the value is positive,
    // then it represents the units per beat. For example, +96 would
    // mean 96 ticks per beat. If the value is negative, delta times
    // are in SMPTEpub division: i16, compatible units.
    midipiece.division = 480; // 1 tick = 24 clocks; let rep_args = replace_args.clone();quarter = 480 ticks; 1/32 = 60 ticks
    let writer = SMFWriter::from_smf(midipiece);
    let path = format!("{}{}.mid", RUSTMIDI_TEST_PATH, mseq.title);
    writer.write_to_file(&Path::new(&path[..])).unwrap();
    //read_file(&path[..]);
      

}


/// Creates a MidiFile with Track 0 as Header, Tracks 1 as Solo Instrument on channel 0.
/// If 'figures' Vector is empty, generates a flux of notes with the same duration (interval_time).
/// 'const RUSTMIDI_TEST_PATH' is the target directory.
fn write_solosequence(seq: SoloSequence) {
    let ms = MultiSequence {
        title: &seq.title[..],
        transpose: vec![],
        instruments: vec![], 
        velocities: vec![],
        interval_time: seq.interval_time,
        bpm: seq.bpm,
        solosequences: vec![
            seq,
        ],
    };
    write_multisequence(ms);
}


/// MidiFile dumping function on terminal
fn _read_file(path: &str) {
    println!("Reading: {}", path);
    match SMF::from_file(&Path::new(&path[..])) {
        Ok(smf) => {
            println!("format: {}", smf.format);
            println!("tracks: {}", smf.tracks.len());
            println!("division: {}", smf.division);
            let mut tnum = 1;
            for track in smf.tracks.iter() {
                let mut time: u64 = 0;
                println!("\n{}: {}\nevents:", tnum, track);
                tnum += 1;
                for event in track.events.iter() {
                    // if time > 2000 {
                    //     break;
                    // }
                    println!("  {}", event.fmt_with_time_offset(time));
                    time += event.vtime;
                }
            }
        }
        Err(e) => match e {
            SMFError::InvalidSMFFile(s) => {
                println!("{}", s);
            }
            SMFError::Error(e) => {
                println!("io: {}", e);
            }
            SMFError::MidiError(e) => {
                println!("Midi Error: {}", e);
            }
            SMFError::MetaError(_) => {
                println!("Meta Error");
            }
        },
    }
}