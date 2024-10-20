use synth_music::prelude::*;

mod instruments;
mod tracks;

fn main() {
    println!("Melody Example");

    let settings = CompositionSettings {
        sample_rate: 44100,
    };

    let info_normal = SectionInfo {
        bpm: 120.0,
        key: music_key::A_MINOR,
        settings: &settings,
    };

    let info_fast = SectionInfo {
        bpm: 130.0,
        key: music_key::B_FLAT_MINOR,
        settings: &settings,
    };

    let instrument_melody = instruments::Decaying {
        instrument: predefined::TriangleGenerator,
        decay_speed: 1.0,
    };

    let instrument_chords = instruments::Decaying {
        instrument: predefined::SawGenerator,
        decay_speed: 10.0,
    };
    
    let instrument_bass = instruments::HardBass::new(10);
    let drumset = instruments::Drumset::new();

    let melody_intro = tracks::melody_intro(instrument_melody);
    let chords_intro = tracks::chords_intro(instrument_chords);
    let bass_intro = tracks::bass_intro(instrument_bass);

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

    let composition = composition!(
        // Intro
        section!(info_normal,
            melody_intro,
            chords_intro,
            bass_intro,
        ),

        // First repetition
        section!(info_normal,
            melody_begin,
            chords_begin,
            bass_begin,
        ),
        section!(info_normal,
            melody_repeated_first,
            chords_repeated,
            bass_repeated,
        ),
        section!(info_normal,
            melody_repeated_second,
            chords_repeated,
            bass_repeated,
            tracks::drumset_bass(drumset, 4),
        ),

        section!(info_normal,
            melody_b_section_first,
            chords_b_section,
            bass_b_section,
            tracks::drumset_quarterbeat(drumset, 4),
        ),
        section!(info_normal,
            melody_b_section_second,
            chords_b_section,
            bass_b_section,
            tracks::drumset_eightbeat(drumset, 4),
        ),

        section!(info_normal,
            melody_begin,
            chords_begin,
            bass_begin,
            tracks::drumset_quarterbeat(drumset, 4),
        ),
        section!(info_normal,
            melody_repeated_first,
            chords_repeated,
            bass_repeated,
            tracks::drumset_eightbeat(drumset, 4),
        ),
        section!(info_normal,
            melody_repeated_second,
            chords_repeated,
            bass_repeated,
            tracks::drumset_eightbeat(drumset, 4),
        ),

        section!(info_fast,
            melody_begin,
            chords_begin,
            bass_begin,
            tracks::drumset_quarterbeat(drumset, 4),
        ),
        section!(info_fast,
            melody_repeated_first,
            chords_repeated,
            bass_repeated,
            tracks::drumset_eightbeat(drumset, 4),
        ),
        section!(info_fast,
            melody_repeated_second,
            chords_repeated,
            bass_repeated,
            tracks::drumset_eightbeat(drumset, 4),
        ),

        section!(info_fast,
            melody_b_section_first,
            chords_b_section,
            bass_b_section,
            tracks::drumset_quarterbeat(drumset, 4),
        ),
        section!(info_fast,
            melody_b_section_second,
            chords_b_section,
            bass_b_section,
            tracks::drumset_eightbeat(drumset, 4),
        ),

        section!(info_fast,
            melody_begin,
            chords_begin,
            bass_begin,
            tracks::drumset_quarterbeat(drumset, 4),
        ),
        section!(info_fast,
            melody_repeated_first,
            chords_repeated,
            bass_repeated,
            tracks::drumset_eightbeat(drumset, 4),
        ),
        section!(info_fast,
            melody_repeated_second,
            chords_repeated,
            bass_repeated,
            tracks::drumset_bass(drumset, 4),
        ),
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
