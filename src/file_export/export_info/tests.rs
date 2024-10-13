#![cfg(test)]

use super::*;

// Test SoundBuffer

fn assert_soundbuffer_equal(a: SoundBuffer, b: SoundBuffer) {
    assert_eq!(a.sample_rate, b.sample_rate);
    assert_eq!(a.active_samples, b.active_samples);
    assert_eq!(a.samples, b.samples);
}

// Tests for extend_to_active_samples()

#[test]
fn soundbuffer_extend_none() {
    let mut soundbuffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        2,
    );

    soundbuffer.extend_to_active_samples();

    let expected = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        2,
    );

    assert_soundbuffer_equal(soundbuffer, expected);
}

#[test]
fn soundbuffer_extend_none_equal() {
    let mut soundbuffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        3,
    );

    soundbuffer.extend_to_active_samples();

    let expected = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        3,
    );

    assert_soundbuffer_equal(soundbuffer, expected);
}

#[test]
fn soundbuffer_extend_active() {
    let mut soundbuffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        5,
    );

    soundbuffer.extend_to_active_samples();

    let expected = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.0, 0.0],
        44100,
        5,
    );

    assert_soundbuffer_equal(soundbuffer, expected);
}

// Tests for mix()

#[test]
fn soundbuffer_mix_simple() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        3,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        44100,
        3,
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
        ],
        44100,
        3
    );

    assert_soundbuffer_equal(result, expected);
}

#[test]
fn soundbuffer_mix_partial_full() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4],
        44100,
        4,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        44100,
        3,
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
            0.4,
        ],
        44100,
        4
    );

    assert_soundbuffer_equal(result, expected);
}

#[test]
fn soundbuffer_mix_partial_half_full() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4],
        44100,
        3,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        44100,
        3,
    );

    let result = first_buffer.mix(second_buffer);

    let expected = SoundBuffer::new(
        vec![
            0.1 + 0.4,
            0.2 + 0.2,
            0.3 + 0.0,
            0.4
        ],
        44100,
        3
    );

    assert_soundbuffer_equal(result, expected);
}

#[test]
fn soundbuffer_mix_partial_not_full() {
    let first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3, 0.4, 0.5],
        44100,
        2,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0, -0.3],
        44100,
        3,
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
        44100,
        3
    );

    assert_soundbuffer_equal(result, expected);
}

// Tests for append()

#[test]
fn soundbuffer_append_simple() {
    let mut first_buffer = SoundBuffer::new(
        vec![0.1, 0.2, 0.3],
        44100,
        3,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        44100,
        3,
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
        44100,
        1,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        44100,
        3,
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
        44100,
        1,
    );
    let second_buffer = SoundBuffer::new(
        vec![0.4, 0.2, 0.0],
        44100,
        3,
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
