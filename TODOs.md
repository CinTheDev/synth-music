# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

**NOTE**: This document shall now only contain points that are included in the
first release version (0.1.0), the document shall be deleted once the code is
ready for release (when all points are completed). Other ideas for future
versions shall be written in another document.

## Thorough documentation

It seems like the code structure is somewhat converging to a final state, so
documenting the code is a good idea. The README.md also belongs to this. It's
better to do this early while still familiar with the code.

Also make sure to include some examples inline with documentation, research
online what best practises for this are.

### README.md

Fill the readme with useful text as described above.

### More and better examples

All examples right now only serve testing purposes and aren't that great for
showing the library's features. Write a bunch more examples with more isolated
ideas and more comments so these can be used to learn this library.

### Unit tests

It would be good to at least implement the easy unit tests.

### Improve a bit of structure

Only small things should be changed, like introducing constants for important
or meaningful values.

- Introduce constants for Length and use everywhere
- Introduce constants for MusicKey and use in examples
- Make `use` instructions more consistent and efficient

### Seperate future TODOs

All my ideas for future versions are located in `TODOs-future.md`. These ideas
should be seperated into `TODOs-1.1.md` if they do not require breaking changes,
and into `TODOs-2.0md` if they do require breaking changes.
