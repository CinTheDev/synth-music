# Features / fixes that will introduce breaking changes

A collection of ideas, improvements and refactoring tasks that will introduce
breaking changes. An update that contains breaking changes will increment the
second number of the version.

After completion of a specific task here the description will be deleted.
Though, through versioning (git) it's possible to review finished (and therefore
deleted) tasks. Other ideas for future versions shall be written in another
document.

## Make `beat_emphasis` not an Option<>

Right now, `beat_emphasis` provided with `Tone` is an Option, indicating whether
a tone is an offbeat or not. In practice, the user needs to unwrap and specify
an offbeat intensity every time. It would make more sense if the offbeat
intensity was specified with the TimeSignature, and that `beat_emphasis` becomes
a simple f32.

## Support Stereo

This will probably require some refactoring on the rendering side. Make it
possible for Instruments to output stereo sound and export it stereo. Also think
about ways to make the Instrument implementation convenient if only Mono should
be Output.

## Even more advanced dynamics system

Features for making the dynamics system even more versatile and powerful

### Custom interpolation curves

Currently, if we slowly invrease the intensity, the change is always linear.
Make it possible for the user to specify their own interpolation curve.

Either make them choose from an enum, or let them provide their own function,
or both.

### Dynamics effects over Track or whole Section

My idea of this is still a little vague, but I think in some other music it
sometimes feels like the intensity of some instruments "pulse" or similar with
the rythm.

Investigate online what this is called and how it is usually implemented. If
it's not very difficult, implement it here as well.

## Better multithreading

Right now multithreading only works per-section. The sections themselves are
rendered sequentially. Because of this, multithreading is only really beneficial
if the sections are quite long, or if there are a lot of tracks playing at once.

Since there's no way to store Sections in a struct as of right now, it's pretty
difficult to write extensive features for them. If Sections can actually be
bundled together somehow at some point, then it would be feasible to implement
this composition-wide multithreading.

Also while we're at this - if we manage to get outside of macro territory we
might as well clean up the code by moving multithreading into dedicated
functions or even submodules.
