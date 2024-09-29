use unnamed_music::melody::prelude::*;

fn main() {
    println!("Hello example");

    example_1();
}

fn example_1() {
    let key = MusicKey {
        tonic: KeyTonic::C,
        key_type: KeyType::Major,
    };

    let info = SectionInfo {
        bpm: 120.0,
        key,
        time_signature: (4, 4),
    };

    let instrument = SineGenerator;

    let track = {
        use note::scaled_value::*;
        use note::Length::*;
        let mut track = Track::new(Box::new(instrument));

        track.note(Quarter, first(4));
        track.note(Quarter, second(4));
        track.note(Quarter, third(4));
        track.note(Quarter, fourth(4));
        track.note(Quarter, fifth(4));
        track.note(Quarter, sixth(4));
        track.note(Quarter, seventh(4));
        track.note(Quarter, first(5));

        track
    };

    let section = Section {
        info,
        tracks: vec![track],
    };

    let composition = Composition {
        sections: vec![section],
    };

    export(composition.to_export_piece(), "first_example.wav");
}

fn export(export_piece: ExportMusicPiece, name: &str) {
    use unnamed_music::file_export::*;
    use wav_export::WavExport;
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let music_buffer = MusicBuffer::new(export_piece);
    let path = PathBuf::from("export").join(name);

    let exporter = WavExport {
        path,
        sample_rate: 44100,
        ..Default::default()
    };

    exporter.export(music_buffer).unwrap();
}

#[derive(Clone, Copy)]
struct SineGenerator;

impl Instrument for SineGenerator {
    fn generate_sound(&self, info: ToneInfo) -> f32 {
        use std::f32::consts::PI;

        let frequency = info.tone.to_frequency();

        return (info.time.as_secs_f32() * frequency * 2.0 * PI).sin();
    }
}
