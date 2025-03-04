# Changelog

## v0.1.0 [BROKEN]

- Initial release

## v0.1.1

- Fix dependency in `section!` macro

## v0.1.2

- Fix getting the active Measure in MeasureTrack
- Remove redundancy of MeasureTrack by wrapping UnboundTrack

## v0.2.0

- Make the `Instrument` trait more modular and provide default implementations
- Bake `offbeat_intensity` into the general intensity
- Make `get_active_note()` part of `MusicTrack` trait
- Use `SoundBuffer` for Instrument implementation

## v0.2.1

- Provide wave functions with phase
- Add function to normalize a `SoundBuffer`
- Provide noise generation functions
- Provide frequency filter functions, similar to an EQ.
- Add `Curve` trait and `LinearCurve` implementor for intuitively constructing a
graph function (e.g. for frequency filter)
- Move TODOs to GitHub
