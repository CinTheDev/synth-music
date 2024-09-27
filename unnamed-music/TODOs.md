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

## Sound samples

TODOs regarding generation of sound samples

### Find solution for "clicking" between tones

Often when a tone / note starts or stops there is an audible "click" that sounds
unpleasant. This click comes from the audio sample stopping before it smoothly
reaches 0 amplitude.

Find a solution to this problem without pushing it onto the custom instrument
implementation.

#### Idea 1: Differential equations instead functions

One cool thing about differential equations is that they can work independent
of "absolute" time. With "absolute" time I mean e.g. the time since the note
started, which is crucial for the formula approach. A differential equation
only needs the current amplitude to determine the next point.

This gurantees that the amplitude won't jump around when switching frequencies.
There are just two problems with this approach:

**Problem 1**: This approach needs a starting value to work. If we just set the
first sample to our starting value, that will be a jump and a click will be
heard. Maybe it's possible to make a differential equation that can start from
zero amplitude, but that's already fancy stuff.

**Problem 2**: If the sampling stops (e.g. end of section) the amplitude has
not time to go back to zero, and thus it might jump somewhere. This could be
avoided by not resetting the amplitude on section transitions, but that might
turn out to be difficult.

#### Idea 2: Fade in & Fade out

To not make the amplitude change so harsh, we could implement a simple fade in
and fadeout for every track. The fade out should approach zero amplitude to
ensure the transition is smooth.

This idea should work nicely for most cases because it really prevents amplitude
jumps if everything is handled correctly. The fade-in-time could also change
with note intensity for additional softness / hardness in the tone.

This shall be implemented at export-conversion level and not at instrument
level. Just make sure the code doesn't become really messy for this. (Though,
this code section kinda needs to be reworked anyway).

## Support multiple tracks

Right now only one track and one instrument can be used. Improve implementation
so that any number of tracks is possible.
