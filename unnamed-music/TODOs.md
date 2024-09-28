# TODOs

All TODOs of this project are written here when needed; after completion the
relevant points will be removed again. Though, through versioning (git) it's
possible to still read finished (and therefore deleted) points.

## Missing features for composing

There are some missing features that are crucial for composing music.

### Intervals and pauses

For now, the function to place a note only takes a single value. This is by
design to be as short as possible. Though, tones are actually stored as vectors
to be able to represent intervals or even pauses.

Convert the note placement function into a macro so the amount of parameters
is variable. With this it should be possible to provide multiple tones or
even no tones to represent a pause.

### Seperation of measures

Measures are kind of invisible/non-existant in code right now. This isn't the
end of the world, the code works perfectly fine without it. Though when it comes
to workflow, it's a bit unusual to have no measures.

Think about a workflow feature to visibly seperate notes out into measures.

### Crescendo and decrescendo

A (de)crescendo represents a transition of the intensity. Since a (de)crescendo
should be able to change the intensity while a note plays, it needs to be
represented individually.

Implement it and restructure if needed.

## Music Key & Notes cleanup

Right now notes, tones, and music keys have been implemented as closly to
traditional music theory as possible. The problem is that traditional music
theory is kinda inconsistent with definitions sometimes, and the code looks
really messy.

Try to make the implementations for getting the note frequencies more efficient
and readable, ideally using as few magic numbers and long match lists as
possible. Also try to relocate the code to a better place than `composer.rs`.

## Support multiple tracks

Right now only one track and one instrument can be used. Improve implementation
so that any number of tracks is possible.
