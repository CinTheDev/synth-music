use synth_music::prelude::*;

fn main() {

}

fn track_four_four() -> MeasureTrack<TET12ScaledTone, predefined::TriangleGenerator> {
    use tet12::*;
    use length::*;

    let time_signature =
        TimeSignature::new(4, 4)
        .set_beat(0, 1.1)
        .set_beat(2 , 1.05);

    let mut track = MeasureTrack::new(predefined::TriangleGenerator, time_signature);

    for _ in 0..4 {
        sequential_notes!(track, QUARTER,
            first(3),
            first(3),
            first(3),
            first(3),
        );
        track.measure().unwrap();
    }

    track.pause(WHOLE);
    track.measure().unwrap();

    return track;
}

fn track_three_four() -> MeasureTrack<TET12ScaledTone, predefined::TriangleGenerator> {
    use tet12::*;
    use length::*;

    let time_signature =
        TimeSignature::new(3, 4)
        .set_beat(0, 1.1);

    let mut track = MeasureTrack::new(predefined::TriangleGenerator, time_signature);

    for _ in 0..4 {
        sequential_notes!(track, QUARTER,
            third(3),
            third(3),
            third(3),
        );
        track.measure().unwrap();
    }

    track.pause(HALF.dot());
    track.measure().unwrap();

    return track;
}

fn track_six_eight() -> MeasureTrack<TET12ScaledTone, predefined::TriangleGenerator> {
    use tet12::*;
    use length::*;

    let time_signature =
        TimeSignature::new(6, 8)
        .set_beat(0, 1.1)
        .set_beat(3 , 1.05);

    let mut track = MeasureTrack::new(predefined::TriangleGenerator, time_signature);

    for _ in 0..4 {
        sequential_notes!(track, QUARTER,
            first(3),
            first(3),
            first(3),

            third(3),
            third(3),
            third(3),
        );
        track.measure().unwrap();
    }

    track.pause(HALF.dot());
    track.measure().unwrap();

    return track;
}
