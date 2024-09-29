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

        track.note(Quarter, first(0));
        track.note(Quarter, second(0));
        track.note(Quarter, third(0));
        track.note(Quarter, fourth(0));
        track.note(Quarter, fifth(0));
        track.note(Quarter, sixth(0));
        track.note(Quarter, seventh(0));
        track.note(Quarter, first(1));

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
        todo!()
    }
}
