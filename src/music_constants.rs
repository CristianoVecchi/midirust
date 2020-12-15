#![allow(dead_code)]
pub const IONIAN: [i32;7] = [UNISON, MAJ_2, MAJ_3, P_4, P_5, MAJ_6, MAJ_7];
pub const DORIAN: [i32;7] = [UNISON, MAJ_2, MIN_3, P_4, P_5, MAJ_6, MIN_7];
pub const PHRYGIAN: [i32;7] = [UNISON, MIN_2, MIN_3, P_4, P_5, MIN_6, MIN_7];
pub const LYDIAN: [i32;7] = [UNISON, MAJ_2, MAJ_3, AUM_4, P_5, MAJ_6, MAJ_7];
pub const MIXOLYDIAN: [i32;7] = [UNISON, MAJ_2, MAJ_3, P_4, P_5, MAJ_6, MIN_7];
pub const EOLIAN: [i32;7] = [UNISON, MAJ_2, MIN_3, P_4, P_5, MIN_6, MIN_7];
pub const LOCRIAN: [i32;7] = [UNISON, MIN_2, MIN_3, P_4, DIM_5, MIN_6, MIN_7];
pub const DOUBLE_HARMONIC: [i32;7] = [UNISON, MIN_2, MAJ_3, P_4, P_5, MIN_6, MAJ_7];
pub const EXATONIC: [i32;6] = [UNISON, MAJ_2, MAJ_3, AUM_4, AUM_5, MIN_7];
pub const AUGMENTED: [i32;6] = [UNISON, MIN_3, MAJ_3, P_5, AUM_5, MAJ_7];
pub const BLUES: [i32;6] = [UNISON, MIN_3, P_4, DIM_5, P_5, MIN_7];
pub const PENTATONIC_MAJOR: [i32;5] = [UNISON, MAJ_2, MAJ_3, P_5, MAJ_6];
pub const PENTATONIC_MINOR: [i32;5] = [UNISON, MIN_3, P_4, P_5, MIN_7];
pub const OCTATONIC_12: [i32;8] = [UNISON, MIN_2, MIN_3, MAJ_3, AUM_4, P_5, MAJ_6, MIN_7];
pub const OCTATONIC_21: [i32;8] = [UNISON, MAJ_2, MIN_3, P_4, AUM_4, AUM_5, MAJ_6, MAJ_7];

pub const DOUBLE_WHOLE: i32 = 3840;
pub const WHOLE: i32 = 1920;
pub const HALF: i32 = 960;
pub const QUARTER: i32 = 480;
pub const EIGHTH: i32 = 240;
pub const SIXTEENTH: i32 = 120;
pub const THIRTY_SECOND: i32 = 60;
pub const SIXTY_FOURTH: i32 = 30;

pub const BREVE: i32 = 3840;
pub const SEMIBREVE: i32 = 1920;
pub const MINIM: i32 = 960;
pub const CROTCHET: i32 = 480;
pub const QUAVER: i32 = 240;
pub const SEMIQUAVER: i32 = 120;
pub const DEMISEMIQUAVER: i32 = 60;
pub const HEMIDEMISEMIQUAVER: i32 = 30;

pub const REST: i32 = -1;

pub const HALF_STEP: i32 = 1;
pub const SEMITONE: i32 = 1;
pub const WHOLE_STEP: i32 = 2;
pub const TONE: i32 = 2;

pub const MAJ_2: i32 = 2;
pub const MIN_2: i32 = 1;
pub const UNISON: i32 = 0;
pub const MAJ_3: i32 = 4;
pub const MIN_3: i32 = 3;
pub const MAJ_6: i32 = 9;
pub const MIN_6: i32 = 8;
pub const MAJ_7: i32 = 11;
pub const MIN_7: i32 = 10;
pub const MAJ_9: i32 = 14;
pub const MIN_9: i32 = 13;
pub const MAJ_10: i32 = 16;
pub const MIN_10: i32 = 15;
pub const OCTAVE: i32 = 12;
pub const P_4: i32 = 5;
pub const AUM_4: i32 = 6;
pub const DIM_5: i32 = 6;
pub const AUM_5: i32 = 8;
pub const P_5: i32 = 7;
pub const P_11: i32 = 17;
pub const P_12: i32 = 19;
pub const AUG_11: i32 = 18;
pub const MAJ_13: i32 = 21;
pub const MIN_13: i32 = 20;
