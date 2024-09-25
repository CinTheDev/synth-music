# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

## File Writer

TODOs regarding the file_writer submodule

### Use dedicated "Buffer" struct to store buffer

Will improve:

- Memory consumption (by a lot)
- Abstraction from technicalities

Instead of storing the buffer completely as a byte array, it should be
represented as a struct. The struct does not need to have the entire buffer
saved in memory at the time it is given to the export function, it just needs
to have all the values to deterministically generate a chunk of the sound.

This buffer shall implement a function to generate and return a byte buffer
of a specific time segment in a given time interval. This buffer is then used
by export to be written into a file periodically until the entire buffer is
exhausted.

## Melodies

TODOs regarding the melodies submodule

### Base of Melodies

Create the base of the submodule and integrate it into the library.

## Sound samples

TODOs regarding generation of sound samples

### Base of Sound samples

Create the base of the submodule and integrate it into the library.

## (Temporary section) Structure of Melody

The submodule `melody` contains traits and structs for composing the music.

### Seperation between info and construction

There are two types of info that we need to store: The actual melody the program
will export, and the composition from the user.

The "export info" is basically just a collection of frequencies with their
start times, durations and intensities. The program can generate the music from
those things alone.

The user however assigns many more abstract values that are important to them,
but not to the program. Those things are the music Key, Time Signature, Notes,
and other.

Part of the exporting process would be to convert the composition info into
the much more concrete export info, from which the sound is then generated.

### Piece

Represents the whole music piece which will later be exported. A piece contains
a specifiable number of instruments.

Since the individual melodies are seperated by instrument, it could be difficult
to sight-read the whole piece. To improve readability and maintainability of
the piece, it shall be seperated into multiple "sections".

The start of a section marks the start for all instruments on that section,
if an instrument contains less measures than the other instruments, the
remaining undefined measuers shall be filled in automatically with pauses.

In addition to that, every section shall store the key, BPM, and Time Signature.
This way those values can be changed during playtime.

### Instrument

The instrument contains info about the sound generation (Later when it is
implemented) and multiple tracks for the melody.

### Track

A track is like a single staff containing a melody (or notes). Therefore the
track contains all of the notes in order.

### Note

A single note is represented by its tone (or height), its length measured
in beats, and its intensity or loudness. It should be possible to have multiple
tones in one note.

### Missing features in the description above

Not all features I'd like to implement are described in the structure above.
To not accidentally forget some of them, here's a complete list of all these
features:

- Trioles & dotted notes (probably)
- Seperation of measures
- Pause instead of note
- Accelerando and Ritardando
