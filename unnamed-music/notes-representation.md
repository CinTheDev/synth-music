# Notes Representation

This is a temporary document for helping with the Notes redesign.

## General requirements

A loose list of requirements that the design should handle easily.

### A note can have n number of values

This also includes 0 values. 0 values represent a pause, while multiple values
are a chord or similar.

### Every single value can be sharpened or flattened individually

The emphasis here is that **every single value** can be modified in height.
The old system has the problem that in a chord individual values cannot be
sharpened or flattened.

That means the representation of sharps or flats should be in the value itself,
not in the whole note structure (the note structure holds all notes that play
together).

### The general note length can be modified

This is similar to above with note values, but the note length must be the same
for all values in a chord. The old system did that correctly, so keep it the
same way.

### Notes are stored independent of key

A note value should be representable independently of the section key. This
makes handling notes later on during export or render much easier.

Though, when placing notes during the composition process, the key does matter.
More info on that later.

### Placed notes are abstract values

When placing a note, we don't want to think about the specific value that it is,
because it will change when the key is changed. With the old system these
abstract values are called "First", "Second", up until "Seventh". The "First"
value would be a C if the key is C Major for example.

This system has a flaw though, it contradicts the requirement that individual
values can be modified in height, e.g. sharpened or flattened, unless we
introduce additional values like "FirstSharp". Though, that's kinda messy and
we should look for a better approach.

Independent of that, when notes are placed the composer should directly convert
these to concrete values like "C#" or "A".

### It should be possible to use other (user defined) note systems

This is the biggest requirement yet. Right now, only the standard 12-TET notes
can be used, and because they're directly converted to raw frequencies, they're
not that flexible.

The major issue with this is that if the user wants to use an instrument that
does not operate on notes (e.g. a drumset), it makes no real sense to represent
notes like that.

To fix that, there need to be two major changes:

#### Make notes represented by a trait or similar

So that different note systems can be used.

#### Pass note values to instrument function

Currently the instrument sound function receives a frequency to generate a
sound. With the changes above it would make more sense to pass the concrete
note values to the instrument function.

The conversion from 12-TET note values to frequencies could be handled by an
external function that can be called from the instrument sound function.

### More advanced dynamics representation

The intensity of a note is currently represented only as a single float between
0.0 and 1.0.

Often, there's a term like "acceleration" of tones, which relates to intensity.
The acceleration of a tone describes how "hard" the tone begins to sound.
This behaviour is individual to the instrument, so that must be handled by the
user. *The user shall get the needed information to do such calculations.*

Also, later on we want to implement (de)crescendos, which are basically a smooth
transition of the intensity. Though, in one case there's multiple ways how this
could be handled.

Imagine a long held note, and a crescendo is applied over the duration of the
note. That means that at the beginnning of the note the intensity is low, and
at the end of it the intensity is high.

Now, if the instrument operates on "pulses" (e.g. piano, guitar), the intensity
is determined at the beginning of the note but remains unchanged for the
duration of the note, the crescendo is effectively ignored. Often the notes of
these instruments become quiter when held for longer time.

But if the instrument operates on "lengths" (e.g. strings, brass, woodwinds),
the intensity can change during the note duration. Here the intensity would
slowly increase because of the crescendo. Often (without crescendo) the notes
of these instruments would have constant loudness when held for longer time.

Since both behaviours are possible, even for two instruments playing at the
same time, the type of behaviour should be (easily) determined by the user.

## Representation of ExportTone

The export tones are generated from composition tones, and are given to the
instrument sound generator to render a tone.

An ExportTone represents the following info:

- All concrete Tones
- A *total* duration (time until the next tone)
- A *tone* duration (time for which tone is played)
- An intensity or transition of intensity

Note: the "fade-in" or "fade-out" times should still be applied in rendering,
but it does not need to be stored for every tone individually.

## Representation of Note (Composition Tones)

The Notes are what is stored when composing a piece. These are later converted
into ExportTones.

Note: The placement interface does not necessarily need to match the
representation of a note. E.g.: Notes are stored independent of key, but placing
Notes is not; as a result, stored notes are fundamentally different than placed
notes. Placed notes are not stored.

A Note represents the following info:

- All concrete tones
- A note length (time until next tone)
- A play fraction (fraction for which note is played)
- An intensity or transition of intensity (make it crescendo-friendly)
- Some flags (e.g. if they're dotted, legato, etc.)

## Note placement interface

We will be using macros and functions for constructing notes as described above.
The note structure does not really match the user-given information for the
note, so conversions have to take place directly inside these functions or
macros. There are several parts to this that are described below.

### Concrete note values

A concrete note value is just a note without associated key. In most cases
they're represented as piano keys. These notes are "C", "D", "E", ..., or
other tones such as "Cb", "C#", "Fb", "A#", ...

This is the standard 12-TET note system. Though, the requirements say that we
mustn't force the user to use this note system, so it should rather be an
implementation of a trait or similar.

The issue is that the traditional note names are kinda inconsistent because
the convention uses the C Major / A minor key. Other keys seem to be "altered"
versions of the "standard" values, even though they're independent. On top of
that, there are multiple names that refer to the same note value, like "C#" and
"Db". If we don't want to associate a key with the note value, there's literally
no reason to choose one name over the other. Also, the distance of semitones
from a "standard" key to another is not consistent. Sometimes it's two
semitones, sometimes it's just one.

It's possible to shift concrete note values by a number of semitones. For
implementation it means it would be very inconvinient to represent these as an
enum, since enums cannot be shifted so easily. The doubled note names make this
even more difficult.

It would be much more convinient to represent concrete note values as an
integer. This way every integer can be assigned one specific note value, and
every note value can be assigned an integer. It's easy to do math with them
(although only addition or subtraction make sense here).

**So here comes the official definition**: A concrete note value shall be
represented as an integer value. The value is the number of semitones from
the note A4. Positive values make the note value higher, while negative values
lower it.

### Abstract note values

An abstract note value does not represent a specific key on the piano, as it is
dependent on the applied music key. If we know the music key, an abstract note
value can be converted into a concrete note value.

**Definition**: An abstract note value shall be represented as an integer value.
The value is the number of semitones from the tonic of the applied music key.
Positive values make the note value higher, while negative values lower it.

Octaves require a bit of special handling here. So the "tonic" of a key shall
be as close as possible to the note A4. E.g. the key CMajor has its tonic at
C5.

It's possible to sharpen or flatten an abstract note value by adding or
subtracting 1 from it.

#### Design challenge: Placing notes inside a scale

Since abstract note values are still represented as semitones, we still don't
have our required 7-note system in place for the user. A musical scale consists
of 7 notes (that are determined by the key). The eigth note is actually the
first note, but an octave higher.

These notes shall be labeled "first", "second", and so on until "seventh".

The distance of two such notes is not constant, it can either be two semitones
ore just one. Also, the type of key (Major, Minor) affects the distance between
two specific notes. We want the user to work with the 7-note system, but we want
these to also be abstract notes that are represented as semitones from the tonic
so that we can do math with them.

So, a scale representation of a note shall be effortlessly convertable into an
abstract note.

Since I want the syntax to be short, the conversion to an abstract note must
be as quick and immediate as possible. The best way would be to somehow
represent scale notes as constants which are already assinged an abstract value.

##### Idea 1: Automatically converting minor scales into Major scales

By making sure that the scale representation is always a the same type (e.g.
major), the distances between those notes remains deterministic.

For example: If we write a piece in A minor, the first note would be A, the
second B, and so on. Internally the program converts the scale representation
to the major counterpart which here would be C major. In C major, A is the
sixth note, B is the seventh, and so on. The program can then convert the scale
representation into abstract notes, which are then concerted to concrete notes.

**Problem**: Until this conversion step happens, the note value cannot be
modified, e.g. sharpened. It is complex enough that it would require a function
call, and that would be too much effort for placing a single note.

##### Idea 2: Scale representation as struct or enum variant with implementation

(Personally I like the struct variant better so I'll describe that)

The scale representation of structs could be implemented using a struct. This
struct is not meant to be constructed by the user, but there should be various
defined consts of it that represent the individual notes.

This struct could then contain the offset in semitones it additionally has,
and there could be impl functions for it to adjust this offset.

This struct will then automatically be evaluated into an abstract value.
