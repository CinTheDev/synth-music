# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

**NOTE**: This document shall now only contain points that are included in the
first release version (0.1.0), the document shall be deleted once the code is
ready for release (when all points are completed). Other ideas for future
versions shall be written in another document.

## Release

Once all tasks in this document are finished, it's time to publish this on
<crates.io>. The following things need to be done:

- Delete this document
- Merge and push main
- Create GitHub release
- Upload to crates.io

Once version 0.1.0 has been published, create and push a new branch `v0.1.1`
where the file `TODOs-0.1.1.md` will become `TODOs.md`. This will be where
all work on version 0.1.1 will happen.

## Thorough documentation

It seems like the code structure is somewhat converging to a final state, so
documenting the code is a good idea. The README.md also belongs to this. It's
better to do this early while still familiar with the code.

Also make sure to include some examples inline with documentation, research
online what best practises for this are.

### More and better examples

All examples right now only serve testing purposes and aren't that great for
showing the library's features. Write a bunch more examples with more isolated
ideas and more comments so these can be used to learn this library.

Also, fill in the last references in documentation after examples are finished.

List of all required examples:

#### UnboundTrack and MeasureTrack

- Simple comparision between both

#### Instruments

- Different implementations
- TODO

#### Custom note system

- Drumset example
- Probably similar to what Tetris example is doing

#### Tetris example

- Rename old "melody" folder to "tetris"
- Adjust old tracks with dynamics and everything
- Use MeasureTrack
- Improve the Drumset
- Improve the melody if possible (make more interesting)
