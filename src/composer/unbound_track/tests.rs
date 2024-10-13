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
            beat_emphasis: Some(1.0),
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(D4)],
            play_duration: Duration::from_secs_f32(time_whole * 0.5),
            tone_duration: Duration::from_secs_f32(time_whole * 0.5),
            intensity: 1.0..1.0,
            beat_emphasis: Some(1.0),
        },

        Tone {
            concrete_values: vec![TET12ConcreteTone(E4 - 12)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
            beat_emphasis: Some(1.0),
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(F4 - 12)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
            beat_emphasis: Some(1.0),
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(G4 - 12 + 1)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
            beat_emphasis: Some(1.0),
        },
        Tone {
            concrete_values: vec![TET12ConcreteTone(A4 - 12)],
            play_duration: Duration::from_secs_f32(time_whole * 0.125),
            tone_duration: Duration::from_secs_f32(time_whole * 0.125),
            intensity: 1.0..1.0,
            beat_emphasis: Some(1.0),
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
            beat_emphasis: Some(1.0),
        },
    ];

    assert_eq_tones(&result.tones, &expected_tones);
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
    
    if a.beat_emphasis.is_some() {
        assert_eq_f32(a.beat_emphasis.unwrap(), b.beat_emphasis.unwrap(), epsilon);
    }
    else {
        assert!(a.beat_emphasis.is_none());
        assert!(b.beat_emphasis.is_none());
    }
}

fn assert_eq_f32(a: f32, b: f32, epsilon: f32) {
    let delta = (a - b).abs();
    if delta > epsilon {
        panic!("assertion failed: {} != {}", a, b);
    }
}
