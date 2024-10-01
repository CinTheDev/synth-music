# New Submodule Structure

This is a temporary document made for aiding in rearranging the submodules
in a cleaner way.

## Root

The whole crate consists of three parts:

- Composing music (composer)
- Implementing instrument sounds (instrument)
- Exporting music (file_export)

It would be best if the prelude was also located here.

### Composer

- Composer & Sections
- Tracks
- Notes (struct and traits)
- Music Key

#### Track

#### Note

#### Music Key

### Instrument

### File-Export

- **ExportPiece** structure
- Different file supports (currently only WAV)

#### wav_export

## Missing stuff

Collect all missing pieces of the lib here as to not forget anything. At the
end everything should have a proper place.

- Predefined Instruments
- Predefined Note systems
