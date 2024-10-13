#![cfg(test)]

use super::*;

impl Default for CompositionSettings {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
        }
    }
}

// Test SoundBuffer

fn assert_soundbuffer_equal(a: SoundBuffer, b: SoundBuffer) {
    assert_eq!(a.settings, b.settings);
    assert_eq!(a.active_samples, b.active_samples);
    assert_eq!(a.samples, b.samples);
}

// Tests for extend_to_active_samples()

#[test]
fn soundbuffer_extend_none() {
    let mut soundbuffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        2,
        CompositionSettings::default(),
    );

    soundbuffer.extend_to_active_samples();

    let expected = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        2,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(soundbuffer, expected);
}

#[test]
fn soundbuffer_extend_none_equal() {
    let mut soundbuffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        3,
        CompositionSettings::default(),
    );

    soundbuffer.extend_to_active_samples();

    let expected = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        3,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(soundbuffer, expected);
}

#[test]
fn soundbuffer_extend_active() {
    let mut soundbuffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        5,
        CompositionSettings::default(),
    );

    soundbuffer.extend_to_active_samples();

    let expected = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.0, 0.0],
        5,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(soundbuffer, expected);
}

// Tests for mix()

#[test]
fn soundbuffer_mix_simple() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        3,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        3,
        CompositionSettings::default(),
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
        ],
        3,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(result, expected);
}

#[test]
fn soundbuffer_mix_partial_full() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4],
        4,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        3,
        CompositionSettings::default(),
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
            0.4,
        ],
        4,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(result, expected);
}

#[test]
fn soundbuffer_mix_partial_half_full() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4],
        3,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        3,
        CompositionSettings::default(),
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
            0.4
        ],
        3,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(result, expected);
}

#[test]
fn soundbuffer_mix_partial_not_full() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4, 0.5],
        2,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0, -0.3],
        3,
        CompositionSettings::default(),
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
            0.4 - 0.3,
            0.5,
        ],
        3,
        CompositionSettings::default(),
    );

    assert_soundbuffer_equal(result, expected);
}

// Tests for append()

#[test]
fn soundbuffer_append_simple() {
    let mut first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        3,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        3,
        CompositionSettings::default(),
    );

    first_buffer.append(second_buffer);

    let expected_result = vec![
        0.1,
        0.2,
        0.3,
        0.4,
        0.2,
        0.0
    ];

    assert_eq!(first_buffer.active_samples, 6);

    for i in 0..first_buffer.samples.len() {
        assert_eq!(first_buffer.samples[i], expected_result[i]);
    }
}

#[test]
fn soundbuffer_append_partialmix() {
    let mut first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        1,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        3,
        CompositionSettings::default(),
    );

    first_buffer.append(second_buffer);

    let expected_result = vec![
        0.1,
        0.2 + 0.4,
        0.3 + 0.2,
        0.0
    ];

    assert_eq!(first_buffer.active_samples, 4);

    for i in 0..first_buffer.samples.len() {
        assert_eq!(first_buffer.samples[i], expected_result[i]);
    }
}

#[test]
fn soundbuffer_append_fullmix() {
    let mut first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4, 0.5],
        1,
        CompositionSettings::default(),
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        3,
        CompositionSettings::default(),
    );

    first_buffer.append(second_buffer);

    let expected_result = vec![
        0.1,
        0.2 + 0.4,
        0.3 + 0.2,
        0.4 + 0.0,
        0.5,
    ];

    assert_eq!(first_buffer.active_samples, 4);

    for i in 0..first_buffer.samples.len() {
        assert_eq!(first_buffer.samples[i], expected_result[i]);
    }
}
