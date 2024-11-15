#![cfg(test)]

use std::time::Duration;

use crate::prelude::*;
use crate::prelude::tet12::*;
use crate::prelude::length::*;
use crate::prelude::predefined::SineGenerator as instrument;

const SETTINGS: CompositionSettings = CompositionSettings {
    sample_rate: 44100,
};

const C4: i32 = -9;
const D4: i32 = -7;
const E4: i32 = -5;
const F4: i32 = -4;
const G4: i32 = -2;
const A4: i32 =  0;
const B4: i32 =  2;

#[test]
fn conversion_simple() {
    let mut track = UnboundTrack::new(instrument);
    track.set_intensity(1.0);
    track.set_play_fraction(1.0);

    track.note(QUARTER, first(4));
    track.note(HALF, second(4));

    sequential_notes!(track, EIGTH,
        third(3),
        fourth(3),
        fifth(3).sharp(),
        sixth(3),
    );

    notes!(track, WHOLE,
        first(3),
        third(3),
        fifth(3),
        seventh(3),
    );

    sequential_notes!(track, QUARTER.triole(),
        first(4),
        third(4),
        fifth(4),
    );

    let info = SectionInfo {
        bpm: 120.0,
        key: music_key::C_MAJOR,
        settings: &SETTINGS,
    };

    let result = track.convert_to_export_track(info);
    let time_whole = 2.0;

    let expected_tones = vec![
        Tone {
            concrete_values: vec![TET12ConcreteTone(C4)],
            play_duration: Duration::from_secs_f32(time_whole * 0.25),
            tone_duration: Duration::from_secs_f32(time_whole * 0.25),
            intensity: 1.0..1.0,
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(D4)],
            play_duration: Duration::from_secs_f32(time_whole * 0.5),
            tone_duration: Duration::from_secs_f32(time_whole * 0.5),
            intensity: 1.0..1.0,
        },

        Tone {
            concrete_values: vec![TET12ConcreteTone(E4 - 12)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(F4 - 12)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(G4 - 12 + 1)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(A4 - 12)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
        },

        Tone {
            concrete_values: vec![
                TET12ConcreteTone(C4 - 12),
                TET12ConcreteTone(E4 - 12),
                TET12ConcreteTone(G4 - 12),
                TET12ConcreteTone(B4 - 12),
            ],
            play_duration: Duration::from_secs_f32(time_whole * 1.0),
            tone_duration: Duration::from_secs_f32(time_whole * 1.0),
            intensity: 1.0..1.0,
        },

        Tone {
            concrete_values: vec![TET12ConcreteTone(C4)],
            play_duration: Duration::from_secs_f32(time_whole * 0.5 / 3.0),
            tone_duration: Duration::from_secs_f32(time_whole * 0.5 / 3.0),
            intensity: 1.0..1.0,
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(E4)],
            play_duration: Duration::from_secs_f32(time_whole * 0.5 / 3.0),
            tone_duration: Duration::from_secs_f32(time_whole * 0.5 / 3.0),
            intensity: 1.0..1.0,
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(G4)],
            play_duration: Duration::from_secs_f32(time_whole * 0.5 / 3.0),
            tone_duration: Duration::from_secs_f32(time_whole * 0.5 / 3.0),
            intensity: 1.0..1.0,
        },
    ];

    assert_eq_tones(&result.tones, &expected_tones);
}

#[test]
fn conversion_bpm() {
    let mut track = UnboundTrack::new(instrument);

    track.note(QUARTER, first(4));

    let info_1 = SectionInfo {
        bpm: 120.0,
        key: music_key::C_MAJOR,
        settings: &SETTINGS,
    };
    let info_2 = SectionInfo {
        bpm: 100.0,
        key: music_key::C_MAJOR,
        settings: &SETTINGS,
    };
    let info_3 = SectionInfo {
        bpm: 210.0,
        key: music_key::C_MAJOR,
        settings: &SETTINGS,
    };
    let info_4 = SectionInfo {
        bpm: 32.7,
        key: music_key::C_MAJOR,
        settings: &SETTINGS,
    };

    let duration_1 = Duration::from_secs_f32(60.0 / 120.0).as_secs_f32();
    let duration_2 = Duration::from_secs_f32(60.0 / 100.0).as_secs_f32();
    let duration_3 = Duration::from_secs_f32(60.0 / 210.0).as_secs_f32();
    let duration_4 = Duration::from_secs_f32(60.0 / 32.7).as_secs_f32();

    let result_1 = track.convert_to_export_track(info_1).tones[0].play_duration.as_secs_f32();
    let result_2 = track.convert_to_export_track(info_2).tones[0].play_duration.as_secs_f32();
    let result_3 = track.convert_to_export_track(info_3).tones[0].play_duration.as_secs_f32();
    let result_4 = track.convert_to_export_track(info_4).tones[0].play_duration.as_secs_f32();

    let epsilon = 0.001;

    assert_eq_f32(result_1, duration_1, epsilon);
    assert_eq_f32(result_2, duration_2, epsilon);
    assert_eq_f32(result_3, duration_3, epsilon);
    assert_eq_f32(result_4, duration_4, epsilon);
}

#[test]
fn conversion_dynamics() {
    let mut track = UnboundTrack::new(instrument);
    track.set_intensity(0.5);
    track.set_play_fraction(1.0);

    track.note(QUARTER, first(4));

    track.set_intensity(0.1);
    track.note(HALF, first(4));

    track.start_dynamic_change();
    sequential_notes!(track, QUARTER,
        first(4),
        first(4),
        first(4),
    );
    track.end_dynamic_change(1.0);
    track.start_dynamic_change();
    sequential_notes!(track, QUARTER,
        first(4),
        first(4),
        first(4),
    );
    track.end_dynamic_change(0.5);

    track.start_dynamic_change();
    track.note(QUARTER, first(4));
    track.note(HALF, first(4));
    track.note(EIGTH, first(4));
    track.end_dynamic_change(1.0);

    let info = SectionInfo {
        bpm: 120.0,
        key: music_key::C_MAJOR,
        settings: &SETTINGS,
    };

    let export = track.convert_to_export_track(info);

    let expected_intensities = vec![
        0.5 .. 0.5,
        0.1 .. 0.1,

        0.1 .. 0.4,
        0.4 .. 0.7,
        0.7 .. 1.0,

        1.0 .. 0.83333,
        0.83333 .. 0.66666,
        0.66666 .. 0.5,

        0.5 .. 0.64285,
        0.64285 .. 0.92857,
        0.92857 .. 1.0,
    ];

    let epsilon = 0.01;

    for i in 0..export.tones.len() {
        let tone_intensity = &export.tones[i].intensity;
        let expect_intensity = &expected_intensities[i];

        assert_eq_f32(tone_intensity.start, expect_intensity.start, epsilon);
        assert_eq_f32(tone_intensity.end, expect_intensity.end, epsilon);
    }
}

// Utility functions

fn assert_eq_tones<T>(a: &Vec<Tone<T>>, b: &Vec<Tone<T>>)
where 
    T: PartialEq + core::fmt::Debug
{
    assert_eq!(a.len(), b.len());

    for i in 0..a.len() {
        let tone_a = &a[i];
        let tone_b = &b[i];

        assert_eq_tone(tone_a, tone_b);
    }
}

fn assert_eq_tone<T>(a: &Tone<T>, b: &Tone<T>)
where 
    T: PartialEq + core::fmt::Debug
{
    let epsilon = 0.1;

    assert_eq!(a.concrete_values, b.concrete_values);
    assert_eq_f32(a.play_duration.as_secs_f32(), b.play_duration.as_secs_f32(), epsilon);
    assert_eq_f32(a.tone_duration.as_secs_f32(), b.tone_duration.as_secs_f32(), epsilon);
    assert_eq_f32(a.intensity.start, b.intensity.start, epsilon);
    assert_eq_f32(a.intensity.end, b.intensity.end, epsilon);
}

fn assert_eq_f32(a: f32, b: f32, epsilon: f32) {
    let delta = (a - b).abs();
    if delta > epsilon {
        panic!("assertion failed: {} != {}", a, b);
    }
}
