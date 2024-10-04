# Improved Conversion (with regard to music dynamics)

This document is for designing the new export conversion system.

The "export conversion" is a step to convert a Track with specific flags for
every note into an ExportTrack that holds equivalent information, but in a form
that makes it super easy to render.

The old conversion system is a bit primitive, it just works on every individual
note and does calculations that are possible within one note. With the new
dynamics, we need to perform calculations over multiple notes at once.
