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
    let instrument_drumset = instruments::Drumset::new();

    let melody_begin = tracks::melody_begin(instrument_softbass);
    let chords_begin = tracks::chords_begin(instrument_softbass);
    let bass_begin = tracks::bass_begin(instrument_hardbass);

    let melody_repeated_first = tracks::melody_repeated(instrument_softbass, true);
    let melody_repeated_second = tracks::melody_repeated(instrument_softbass, false);
    let chords_repeated = tracks::chords_repeated(instrument_softbass);
    let bass_repeated = tracks::bass_repeated(instrument_hardbass);

    let melody_b_section_first = tracks::melody_b_section(instrument_softbass, true);
    let melody_b_section_second = tracks::melody_b_section(instrument_softbass, false);
    let chords_b_section = tracks::chords_b_section(instrument_softbass);
    let bass_b_section = tracks::bass_b_section(instrument_hardbass);

    let debug_drums = tracks::debug_drumset(instrument_drumset);

    let mut section_debug_drums = section!(info, 44100, debug_drums);

    let mut section_begin = section!(info, 44100,
        melody_begin,
        chords_begin,
        bass_begin
    );

    let mut section_repeated_first = section!(info, 44100,
        melody_repeated_first,
        chords_repeated.clone(),
        bass_repeated.clone()
    );

    let mut section_repeated_second = section!(info, 44100,
        melody_repeated_second,
        chords_repeated.clone(),
        bass_repeated.clone()
    );

    let mut b_section_first = section!(info, 44100,
        melody_b_section_first,
        chords_b_section.clone(),
        bass_b_section.clone()
    );

    let mut b_section_second = section!(info, 44100,
        melody_b_section_second,
        chords_b_section.clone(),
        bass_b_section.clone()
    );

    let mut composition: Vec<f32> = Vec::new();
    composition.append(&mut section_debug_drums);
    composition.append(&mut section_begin.clone());
    composition.append(&mut section_repeated_first.clone());
    composition.append(&mut section_repeated_second.clone());

    composition.append(&mut b_section_first);
    composition.append(&mut b_section_second);

    composition.append(&mut section_begin);
    composition.append(&mut section_repeated_first);
    composition.append(&mut section_repeated_second);

    export_buffer(composition);
}

fn export_buffer(buffer: Vec<f32>) {
    use std::path::PathBuf;

    if std::fs::read_dir("export").is_err() {
        std::fs::create_dir("export").unwrap();
    }

    let exporter = WavExport {
        path: PathBuf::from("export/tetris.wav"),
        sample_rate: 44100,
        ..Default::default()
    };
    exporter.export(buffer).unwrap();
}
