#![allow(dead_code)]
#[allow(unused_imports)]
use rimd::{
    MidiMessage, SMFBuilder, SMFError, SMFFormat,
    SMFWriter, Track, TrackEvent, SMF,
};

mod replacing_fns;
mod checking_fns;
mod composite_fns;
mod music_constants;
mod note_fns; use note_fns::*;
mod solosequences; use solosequences::*;
mod multisequences; use multisequences::*;
use std::path::Path;
use std::time::Instant;


const RUSTMIDI_TEST_PATH: &str = "/home/cris/Desktop/";
const _RUSTMIDI_TEST: &str = "/home/cris/Desktop/RUSTMIDI_TEST.MID";
const _SINGLE_NOTE: &str = "/home/cris/Desktop/singleNote.mid";
const _BACH_BRAND1: &str = "/home/cris/Desktop/BACH_BRAND1.mid";
const _GENIUS1: &str = "/home/cris/Desktop/Genius1.mid";
const _BACH_INVENTIO1: &str = "/home/cris/Desktop/bach_inventio1.mid";
fn main() {
    let start = Instant::now();
    //let solosequence = "TheAloneLocrio";
    // let solosequence = "TestingFunctions";
    // write_solosequence(get_solosequence(solosequence));
    let multisequence = "MultiSequenceTEST";
    write_multisequence(get_multisequence(multisequence));
   
    let elapsed = start.elapsed();
    println!("Midifile created in {} milliseconds", elapsed.as_millis());
}

// 1_000_000 microseconds == 1 second
fn bpm_to_microseconds(bpm: u32) -> u32 {
    // 1_000_000 : 60 = x : 1/bpm
    (1_000_000 / bpm) * 60
}

fn write_multisequence(mseq: MultiSequence) {
    let mut builder = SMFBuilder::new();
    builder.add_track();
    let mut instrument_names = vec![String::from("Globals")];
    for i in 0..mseq.solosequences.len(){
        instrument_names.push(String::from(format!("Instr. n. = {}", mseq.solosequences[i].instrument)));
    }

    set_header(
        &mut builder,
        0,
        Some(String::from("Copyright2020 by Cristiano Vecchi")),
        Some(String::from(mseq.title)),
        Some(instrument_names),
        Some(bpm_to_microseconds(mseq.bpm)), // micro_second (1_000_000 of a second) in a quarter note (q=60)
        Some([4, 2, 96, 8]),                // 4/4
        Some([0, 0]),                       // C Major
    );
    let n_solosequences = mseq.solosequences.len();
    let mut solosequences = mseq.solosequences;
    
    for tr in 0..n_solosequences{
        builder.add_track();
        let sseq = solosequences.remove(0);
        let volume = MidiMessage::control_change(7, 90, tr as u8); // volume, value of volume, channel
        let instrument = if mseq.instruments.len() > tr {
            MidiMessage::program_change(mseq.instruments[tr], tr as u8) //program, channel
        } else {
            MidiMessage::program_change(sseq.instrument, tr as u8) //program, channel
        };    
        builder.add_midi_abs(tr+1, 0, instrument); // track, time, midimessage
        builder.add_midi_abs(tr+1, 0, volume);
        
        let figures_num = sseq.figures.len();
        let interval = mseq.interval_time;
        let mut xs = sseq.abstract_notes;
        let mut ys = sseq.octaves;
        let mut zs = sseq.figures;
        let velocity = if mseq.velocities.len() > tr {
            mseq.velocities[tr]
        } else {
            sseq.velocity
        }; 
        let transpose = if mseq.transpose.len() > tr {
            mseq.transpose[tr]
        } else {
            0
        }; 
        if figures_num == 0 {
            let result = matrix2d_rnd_generator(sseq.iter, &mut xs, &mut ys);
            let pitches = assign_concrete_pitches_transposing(result, 24, transpose);
            add_notes(
                &mut builder,
                1,
                0,
                pitches,
                velocity,
                0,
                interval,
            ); // 480 = 1/4, 60 = 1/32
        } else {
            let result = matrix3d_rnd_generator(sseq.iter, &mut xs, &mut ys, &mut zs);
            let pairs = result
                .iter()
                .map(|array| [array[0], array[1]])
                .collect::<Vec<[i32; 2]>>();
            let mut durations = result
                .iter()
                .map(|array| array[2] as u64 * interval)
                .collect::<Vec<u64>>();
            let mut pitches = assign_concrete_pitches_transposing(pairs, 24, transpose);
            if sseq.check_n_replace.len() != 0 {
                for c_n_r in sseq.check_n_replace {
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
            add_notes_and_durations(&mut builder, tr + 1, 0, pitches, velocity, tr as u8, durations);
        }
       
    }
    let mut midipiece = builder.result();
    // The unit of time for delta timing. If the value is positive,
    // then it represents the units per beat. For example, +96 would
    // mean 96 ticks per beat. If the value is negative, delta times
    // are in SMPTEpub division: i16, compatible units.
    midipiece.division = 480; // 1 tick = 24 clocks; quarter = 480 ticks; 1/32 = 60 ticks
    let writer = SMFWriter::from_smf(midipiece);
    let path = format!("{}{}.mid", RUSTMIDI_TEST_PATH, mseq.title);
    writer.write_to_file(&Path::new(&path[..])).unwrap();
    //read_file(&path[..]);

}

fn write_solosequence(seq: SoloSequence) {
    let mut builder = SMFBuilder::new();
    builder.add_track(); // header track
    builder.add_track(); // solo instrument track
    set_header(
        &mut builder,
        0,
        Some(String::from("Copyright2020 by Cristiano Vecchi")),
        Some(String::from(seq.title)),
        Some(vec![
            String::from("Globals"),
            String::from(format!("Instr. n. = {}", seq.instrument)),
        ]),
        Some(bpm_to_microseconds(seq.bpm)), // micro_second (1_000_000 of a second) in a quarter note (q=60)
        Some([4, 2, 96, 8]),                // 4/4
        Some([0, 0]),                       // C Major
    );
    let volume = MidiMessage::control_change(7, 100, 0); // volume, value of volume, channel
    let instrument = MidiMessage::program_change(seq.instrument, 0); //program, channel
    builder.add_midi_abs(1, 0, instrument); // track, time, midimessage
    builder.add_midi_abs(1, 0, volume);
    let figures_num = seq.figures.len();
    let velocity = seq.velocity;
    let interval_time = seq.interval_time;
    let interval = seq.interval_time;
    let iter = seq.iter;
    let mut xs = seq.abstract_notes;
    let mut ys = seq.octaves;
    let mut zs = seq.figures;
    if figures_num == 0 { // no rhythm figures, it means a flow of notes with the same duration
        let result_pairs = matrix2d_rnd_generator(iter, &mut xs, &mut ys);
        let pitches = assign_concrete_pitches(result_pairs, 24); // 24 is C1, the lowest C note on the piano keyboard
        add_notes(
            &mut builder,
            1,
            0,
            pitches,
            velocity,
            0,
            interval_time,
        ); 
    } else {
        let result_triples = matrix3d_rnd_generator(iter, &mut xs, &mut ys, &mut zs);
        let pairs = result_triples
            .iter()
            .map(|array| [array[0], array[1]])
            .collect::<Vec<[i32; 2]>>();
        let mut durations = result_triples
            .iter()
            .map(|array| array[2] as u64 * interval)
            .collect::<Vec<u64>>();
        let mut pitches = assign_concrete_pitches(pairs, 24);
        if seq.check_n_replace.len() != 0 {
            for c_n_r in seq.check_n_replace {
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
        add_notes_and_durations(&mut builder, 1, 0, pitches, velocity, 0, durations);
        
    };
    let mut midipiece = builder.result();
    // The unit of time for delta timing. If the value is positive,
    // then it represents the units per beat. For example, +96 would
    // mean 96 ticks per beat. If the value is negative, delta times
    // are in SMPTEpub division: i16, compatible units.
    midipiece.division = 480; // 1 tick = 24 clocks; quarter = 480 ticks
    let writer = SMFWriter::from_smf(midipiece);
    let path = format!("{}{}.mid", RUSTMIDI_TEST_PATH, seq.title);
    writer.write_to_file(&Path::new(&path[..])).unwrap();
    //read_file(&path[..]);
}


#[allow(dead_code)]
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
