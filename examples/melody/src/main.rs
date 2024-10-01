use unnamed_music::prelude::*;

mod instruments;
mod tracks;

fn main() {
    println!("Melody Example");

    let key = MusicKey {
        tonic: KeyTonic::A,
        key_type: KeyType::Minor,
    };

    let info = SectionInfo {
        bpm: 120.0,
        key,
        time_signature: (4, 4),
    };

    let instrument_softbass = instruments::SoftBass::new(1.0);
    let instrument_hardbass = instruments::HardBass::new(10);

    let melody_begin = tracks::melody_begin(Box::new(instrument_softbass));
    let chords_begin = tracks::chords_begin(Box::new(instrument_softbass));
    let bass_begin = tracks::bass_begin(Box::new(instrument_hardbass));

    let melody_repeated_first = tracks::melody_repeated(Box::new(instrument_softbass), true);
    let melody_repeated_second = tracks::melody_repeated(Box::new(instrument_softbass), false);
    let chords_repeated = tracks::chords_repeated(Box::new(instrument_softbass));
    let bass_repeated = tracks::bass_repeated(Box::new(instrument_hardbass));

    let melody_b_section_first = tracks::melody_b_section(Box::new(instrument_softbass), true);
    let melody_b_section_second = tracks::melody_b_section(Box::new(instrument_softbass), false);
    let chords_b_section = tracks::chords_b_section(Box::new(instrument_softbass));
    let bass_b_section = tracks::bass_b_section(Box::new(instrument_hardbass));

    let section_begin = Section {
        info,
        tracks: vec![
            melody_begin,
            chords_begin,
            bass_begin,
        ],
    };

    let section_repeated_first = Section {
        info,
        tracks: vec![
            melody_repeated_first,
            chords_repeated.clone(),
            bass_repeated.clone(),
        ],
    };

    let section_repeated_second = Section {
        info,
        tracks: vec![
            melody_repeated_second,
            chords_repeated.clone(),
            bass_repeated.clone(),
        ],
    };

    let b_section_first = Section {
        info,
        tracks: vec![
            melody_b_section_first,
            chords_b_section.clone(),
            bass_b_section.clone(),
        ],
    };
    let b_section_second = Section {
        info,
        tracks: vec![
            melody_b_section_second,
            chords_b_section.clone(),
            bass_b_section.clone(),
        ],
    };

    let composition = Composition {
        sections: vec![
            section_begin.clone(),
            section_repeated_first.clone(),
            section_repeated_second.clone(),

            b_section_first,
            b_section_second,

            section_begin,
            section_repeated_first,
            section_repeated_second,
        ],
    };

    let export_piece = composition.to_export_piece();
    export(export_piece);
}

fn export<T: Instrument>(export_piece: export_info::ExportMusicPiece<T>) {
    use unnamed_music::file_export::*;
    use wav_export::WavExport;
    use std::path::PathBuf;

    let music_buffer = MusicBuffer::new(export_piece);
    let exporter = WavExport {
        path: PathBuf::from("export/debug.wav"),
        sample_rate: 44100,
        ..Default::default()
    };

    exporter.export(music_buffer).unwrap();
}
