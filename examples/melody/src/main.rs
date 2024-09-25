use unnamed_music::melody::prelude::*;

fn main() {
    println!("Melody Example");

    let track_1 = track_1();
}

fn track_1() -> Track {
    use note::Tone::*;
    let mut track = Track::new();

    track.note(First);
    track.note(Second);
    track.note(Third);
    track.note(Fourth);
    track.note(Fith);
    track.note(Sixth);
    track.note(Seventh);

    return track;
}
