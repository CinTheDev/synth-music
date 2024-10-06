# New section principles

As part of reworking the render system, I want to write down the new way
Sections are defined both for the user and for the library.

## Sections cannot hold Tracks

In the past I already determined that it's really difficult to store the
ExportTracks together in a struct. The Tracks are generic and we must be able
to store them independent of type. `dyn` also doesn't work because the common
impl needs to have some type configured.

The only way to "store" Tracks inside Sections is to directly render them out
right away to reduce them to a buffer. We could store every buffer, but that's
pointless so we'll mix them into one buffer right away. So a Section contains
a single buffer representing all Tracks.

Essentially, we're forced to render Tracks already when bundling them into
Sections. I'll have to look how to make this work with other features in the
future.

### Idea: Including rendering features when bundling sections

Examples for the "features": Chunked Rendering, Loading Bar, ...

Since we're rendering sections when creating them, let's just add all the
features there. E.g. for loading bar: print out section name or something
similar and show progress as a percentage. Future sections are impossible to
include in this print.

## More versatility with length

One special thing I want is that Sections can have more samples than they
should. Before, e.g. if a Section takes 1 second with sample rate 100Hz, it
should have 100 samples. The next section would then just be appended to this
one.

Now if that section has 120 samples instead (e.g. for reverb), we cannot
directly append other sections without distorting time. The next section should
be mixed with the last 20 samples, and then the rest appended.

This means that sections should somehow store their time information so that
they can be merged correctly.
