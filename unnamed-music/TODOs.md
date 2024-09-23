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
