use crate::music_constants::*;
use crate::solosequences::*;

pub struct MultiSequence<'a> {
    pub title: &'a str,
    pub transpose: Vec<i8>,
    pub instruments: Vec<u8>,
    pub velocities: Vec<u8>,
    pub interval_time: i32,
    pub bpm: u32,
    pub solosequences: Vec<SoloSequence<'a>>,
}
pub fn get_multisequence(ms_title: &str) -> MultiSequence {
    let mut multisequences: Vec<MultiSequence> = vec![MultiSequence {
        title: "Pastoral",
        transpose: vec![12, 0, -12],
        instruments: vec![73, 71, 70], // Fl, Cl, Bsn
        velocities: vec![105, 90, 97],
        interval_time: SIXTEENTH,
        bpm: 36, // QUARTERS in a minute
        solosequences: vec![
            get_solosequence("PastoralSolo"),
            get_solosequence("PastoralSolo"),
            get_solosequence("PastoralSolo"),
            
        ],
    }];
    multisequences.retain(|ms| ms.title == ms_title);
    multisequences.remove(0)
}

// 0	Piano	Acoustic Grand Piano
// 1	Piano	Bright Acoustic Piano
// 2	Piano	Electric Grand Piano
// 3	Piano	Honky-tonk Piano
// 4	Piano	Rhodes Piano
// 5	Piano	Chorused Piano
// 6	Piano	Harpsichord
// 7	Piano	Clavinet
// 8	Chromatic Percussion	Celesta
// 9	Chromatic Percussion	Glockenspiel
// 10	Chromatic Percussion	Music box
// 11	Chromatic Percussion	Vibraphone
// 12	Chromatic Percussion	Marimba
// 13	Chromatic Percussion	Xylophone
// 14	Chromatic Percussion	Tubular Bells
// 15	Chromatic Percussion	Dulcimer
// 16	Organ	Hammond Organ
// 17	Organ	Percussive Organ
// 18	Organ	Rock Organ
// 19	Organ	Church Organ
// 20	Organ	Reed Organ
// 21	Organ	Accordion
// 22	Organ	Harmonica
// 23	Organ	Tango Accordion
// 24	Guitar	Acoustic Guitar (nylon)
// 25	Guitar	Acoustic Guitar (steel)
// 26	Guitar	Electric Guitar (jazz)
// 27	Guitar	Electric Guitar (clean)
// 28	Guitar	Electric Guitar (muted)
// 29	Guitar	Overdriven Guitar
// 30	Guitar	Distortion Guitar
// 31	Guitar	Guitar Harmonics
// 32	Bass	Acoustic Bass
// 33	Bass	Electric Bass (finger)
// 34	Bass	Electric Bass (pick)
// 35	Bass	Fretless Bass
// 36	Bass	Slap Bass 1
// 37	Bass	Slap Bass 2
// 38	Bass	Synth Bass 1
// 39	Bass	Synth Bass 2
// 40	Strings	Violin
// 41	Strings	Viola
// 42	Strings	Cello
// 43	Strings	Contrabass
// 44	Strings	Tremolo Strings
// 45	Strings	Pizzicato Strings
// 46	Strings	Orchestral Harp
// 47	Strings	Timpani
// 48	Ensemble	String Ensemble 1
// 49	Ensemble	String Ensemble 2
// 50	Ensemble	Synth Strings 1
// 51	Ensemble	Synth Strings 2
// 52	Ensemble	Choir Aahs
// 53	Ensemble	Voice Oohs
// 54	Ensemble	Synth Voice
// 55	Ensemble	Orchestra Hit
// 56	Brass	Trumpet
// 57	Brass	Trombone
// 58	Brass	Tuba
// 59	Brass	Muted Trumpet
// 60	Brass	French Horn
// 61	Brass	Brass Section
// 62	Brass	Synth Brass 1
// 63	Brass	Synth Brass 2
// 64	Reed	Soprano Sax
// 65	Reed	Alto Sax
// 66	Reed	Tenor Sax
// 67	Reed	Baritone Sax
// 68	Reed	Oboe
// 69	Reed	English Horn
// 70	Reed	Bassoon
// 71	Reed	Clarinet
// 72	Pipe	Piccolo
// 73	Pipe	Flute
// 74	Pipe	Recorder
// 75	Pipe	Pan Flute
// 76	Pipe	Bottle Blow
// 77	Pipe	Shakuhachi
// 78	Pipe	Whistle
// 79	Pipe	Ocarina
// 80	Synth Lead	Lead 1 (square)
// 81	Synth Lead	Lead 2 (sawtooth)
// 82	Synth Lead	Lead 3 (calliope lead)
// 83	Synth Lead	Lead 4 (chiffer lead)
// 84	Synth Lead	Lead 5 (charang)
// 85	Synth Lead	Lead 6 (voice)
// 86	Synth Lead	Lead 7 (fifths)
// 87	Synth Lead	Lead 8 (brass + lead)
// 88	Synth Pad	Pad 1 (new age)
// 89	Synth Pad	Pad 2 (warm)
// 90	Synth Pad	Pad 3 (polysynth)
// 91	Synth Pad	Pad 4 (choir)
// 92	Synth Pad	Pad 5 (bowed)
// 93	Synth Pad	Pad 6 (metallic)
// 94	Synth Pad	Pad 7 (halo)
// 95	Synth Pad	Pad 8 (sweep)
// 96	Synth Effects	FX 1 (rain)
// 97	Synth Effects	FX 2 (soundtrack)
// 98	Synth Effects	FX 3 (crystal)
// 99	Synth Effects	FX 4 (atmosphere)
// 100	Synth Effects	FX 5 (brightness)
// 101	Synth Effects	FX 6 (goblins)
// 102	Synth Effects	FX 7 (echoes)
// 103	Synth Effects	FX 8 (sci-fi)
// 104	Ethnic	Sitar
// 105	Ethnic	Banjo
// 106	Ethnic	Shamisen
// 107	Ethnic	Koto
// 108	Ethnic	Kalimba
// 109	Ethnic	Bagpipe
// 110	Ethnic	Fiddle
// 111	Ethnic	Shana
// 112	Percussive	Tinkle Bell
// 113	Percussive	Agogo
// 114	Percussive	Steel Drums
// 115	Percussive	Woodblock
// 116	Percussive	Taiko Drum
// 117	Percussive	Melodic Tom
// 118	Percussive	Synth Drum
// 119	Percussive	Reverse Cymbal
// 120	Sound Effects	Guitar Fret Noise
// 121	Sound Effects	Breath Noise
// 122	Sound Effects	Seashore
// 123	Sound Effects	Bird Tweet
// 124	Sound Effects	Telephone Ring
// 125	Sound Effects	Helicopter
// 126	Sound Effects	Applause
// 127	Sound Effects	Gunshot
