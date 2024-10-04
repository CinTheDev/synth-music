# Improved Conversion (with regard to music dynamics)

This document is for designing the new export conversion system.

The "export conversion" is a step to convert a Track with specific flags for
every note into an ExportTrack that holds equivalent information, but in a form
that makes it super easy to render.

The old conversion system is a bit primitive, it just works on every individual
note and does calculations that are possible within one note. With the new
dynamics, we need to perform calculations over multiple notes at once.

## Idea 1: Multiple passes

Right now, the system only does one pass over the notes and calculates
everything. We could make it do multiple passes where it will calculate a
specific thing and applies it to the result vector.

Problem 1: We still have the issue that we need information from multiple notes
to go into one note.

Problem 2: There's a chance of repeating the same calculation if we need the
same info for multiple different things.

### Potential fixes

Problem 2 is probably easy to fix (at least with syntax). If we make sure to
cleanly sort the calculation into dedicated functions, we won't have to repeat
implementation. Though, we would still be repeating calculations in that case,
but we don't aim for extreme performance, so it's OK for now.

Prolem 1 is a bit harder to think about and to find a solution for, even with
other potential approaches.
