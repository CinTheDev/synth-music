use synth_music::prelude::*;

mod instruments;
mod tracks;

fn main() {
    println!("Melody Example");

    let key = MusicKey {
        tonic: KeyTonic::A,
        key_type: KeyType::Minor,
    };

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let info = SectionInfo {
        bpm: 120.0,
        key,

        settings: &settings,
    };

    let instrument_melody = instruments::SoftBass::new(1.0);
    let instrument_chords = instruments::SoftBass::new(10.0);
    let instrument_bass = instruments::HardBass::new(10);
    let drumset = instruments::Drumset::new();

    let melody_begin = tracks::melody_begin(instrument_melody);
    let chords_begin = tracks::chords_begin(instrument_chords);
    let bass_begin = tracks::bass_begin(instrument_bass);

    let melody_repeated_first = tracks::melody_repeated(instrument_melody, true);
    let melody_repeated_second = tracks::melody_repeated(instrument_melody, false);
    let chords_repeated = tracks::chords_repeated(instrument_chords);
    let bass_repeated = tracks::bass_repeated(instrument_bass);

    let melody_b_section_first = tracks::melody_b_section(instrument_melody, true);
    let melody_b_section_second = tracks::melody_b_section(instrument_melody, false);
    let chords_b_section = tracks::chords_b_section(instrument_chords);
    let bass_b_section = tracks::bass_b_section(instrument_bass);

    let section_begin = section!(info,
        melody_begin,
        chords_begin,
        //bass_begin,
        //tracks::drumset_4(instrument_drumset, 4)
    );

    let section_repeated_first = section!(info,
        melody_repeated_first,
        chords_repeated,
        //bass_repeated,
        //tracks::drumset_4(instrument_drumset, 4)
    );

    let section_repeated_second = section!(info,
        melody_repeated_second,
        chords_repeated,
        //bass_repeated,
        //tracks::drumset_4(instrument_drumset, 4)
    );

    let b_section_first = section!(info,
        melody_b_section_first,
        chords_b_section,
        bass_b_section,
        tracks::drumset_4(drumset, 4)
    );

    let b_section_second = section!(info,
        melody_b_section_second,
        chords_b_section,
        bass_b_section,
        tracks::drumset_4(drumset, 4)
    );

    let composition = composition!(
        section_begin,
        section_repeated_first,
        section_repeated_second,

        b_section_first,
        b_section_second,

        section_begin,
        section_repeated_first,
        section_repeated_second
    );

    export_buffer(composition);
}

fn export_buffer(buffer: SoundBuffer) {
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let exporter = WavExport {
        path: PathBuf::from("export/tetris.wav"),
        ..Default::default()
    };
    exporter.export(buffer).unwrap();
}
