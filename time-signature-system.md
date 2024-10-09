# Time signature design

As described in the TODO, let's think about how to reorganize note length and
time signatures.

## A bit of music theory on time signatures

A time signature essentially contains two different pieces of information: how
"long" a measure is, and how a measure can be subdivided. Possible subdivisions
are doubled, trippled, and quadrupled.

The time signature in music is represented similar to a fraction in math, we
have a nominator and a denominator. In music, the denominator tells us the
"base" note length, while the nominator tells how many of those go in one
measure.

Going further, time signatures are closely related to rythm and beat. The beat
has to do with how the notes are subdivided. The beat describes which notes in
a subdivision are emphasized. In all subdivisions the first note is always
strongly emphasized, and in a quadrupled subdivision the third note is also
weakly emphasized.

### Examples

At this point it's starting to become a little difficult to describe this in a
general way, so I'll continue with examples.

The most common and most stable time signature is 4/4. We have 4 quarter notes
per measure, the first note is strongly emphasized, while the third note is
weakly emphasized.

A very similar time signature is 2/4. There are 2 quarter notes per measure,
where the first one is always emphasized. The difference to 4/4 is that we have
no weakly emphasized third beat, and that measuers are halved in length (but
that's pretty hard to notice in a song).

The time signature 3/4 differs much more from the examples above. While doubled
and quadrupled subdivisions are pretty similar, a trippled subdivision has a
much different vibe. The triple beat makes the song feel elegant and light.

A similar time signature is 6/8, though the difference is much greater than
4/4 to 2/4. The measure feels like it is divided into two segments both with
trippled subdivision. This has a much different quality than 3/4, many people
say it feels like a clock or similar swinging back and forth. I personally find
that description not really fitting, but I can understand where they are coming
from.

Generally, if the denominator is 4 then we can expect *simple time*, and if it
is 8 then we can expect *compound time*.

Simple time is that the notes can be subdivided into groups of two, like for
every quarter of 4/4 there could be two eights, and the same for 4/3, etc.

Now 6/8 is two groups of three eigths. The emphasis is always on the start of
a subdivision, so the first eigth and fourth eight are emphasized.

The same goes for 9/8, now we have 3 groups of each 3 eights.

And then there's *irregular time*. These, as the name already suggests, have
non-predictable and mixed subdivisions in a measure. The most known example for
this is 5/4. It can be divided into a trippled and a doubled group, or the
doubled first and then the trippled. But we can't know from the time signature
alone.

## Converting it into Rust

Well, all this music theory and we must somehow implement it for this library.
Let's go step by step from the ground up and try to be as general as possible
to include as many different configurations as possible.

### Note lengths

Let's start with basic note lengths. Right now they're represented as a simple
enum with some related flags in the Note struct itself. This alone is kind of
bad already: having two different types that should always be together is bad
design.

Also, the enum itself doesn't really represent anything except for a word, and
the logic for every possible state has to be handled individually. If we want
to perform math operations on it (e.g. adding lengths to see how long they are
in combination) requires to convert to a number first, before we can do
operations, and potentially converting it back again afterwards.

This issue looks similar to the note value problem I used to have. Note values
used to also be represented as enums, but later I changed it to be a wrapper
struct around an integer.

So let's do the same for note lengths. We just need a general definition for how
to convert between the integer form and the music form. Unlike notes however,
lengths can be subdivided infinitely (in theory), so integers alone won't be
enough without putting a limit on the subdivisions.

But past 32th notes it's very unlikely that we need more subdivision. So this
representation is valid and should be compared to others. Let's write different
ideas for the representations.

#### Idea 1: Defining a highest level of subdivision

By defining a highest subdivision (like 256th), we can represent lengths as a
single integer. The value 1 would just be a 256th. The value would then
represent the length measured in 256th steps. If we only need to go as far as
a whole note, then an unsigned 8-bit integer would be sufficient.

The value of 255_u8 shall be a whole note, the value of 0_u8 shall be one 256th
note.

Pros:

- Nice and simple
- Fairly good resolution up until 256th notes

Cons:

- Cannot go beyond 1 whole note length
- Fair amount of unnecessary values (nobody needs a 103/256th note)
- Trioles not representable

#### Idea 2: Two integers for subdivision level and count

If we store the subdivision level itself, we don't need to define a lower limit.
The count will be just how many notes of this subdivision we need. It's
sufficient to use u8 integers for both fields. (I feel like it would also work
if we went for a single u8 value with 4 bits reserved for both fields, but we
don't need such an insignificant optimization, it would only save 1 byte per
note at the cost of additional overhead).

The subdivision field shall be the signed number of subdivisions of the note,
0 will be a whole note, 1 will be half, 2 will be quarter, etc... While -1 will
be two whole notes, -2 are 4 whole notes, and so on...

The count field shall be how many of those base lengths form our represented
length. 0 shall be 1 length, 1 shall be 2 lengths, etc.

So if we store it as (count, subdivision), a quarter note would be (0, 2).
An eigth will be (0, 3). A dotted quarter note is represented by (2, 3) (three
eights merged together).

Pros:

- Crazily deep subdivision possible (up until ~ 1 / (1.7 \* 10^38) if quarter is 1/4)
- Also goes up very far (256 \* 3.4\*10^38 whole notes possible)

Cons:

- Wide range of redundant and unnecessary values
- Still no trioles

#### Idea 3: Using floats

Floats can represent many decimal values, which makes handling the subdivisions
extremely easy. We can also directly perfom math operations without much
thinking, because the float represents a note length more concretely than the
ideas above.

The big advantage of floats is that trioles, or even n-toles are possible
without special handling. Trioles are really difficult for the ideas above,
which makes this idea even more powerful. They can even go for very fine
subdivision or very broad and long lengths.

But the downfall of floats is floating-point-imprecision. While the integer
approaches are absolutely precise, float calculations are always just an
approximation. Chained operations will accumulate the error.

E.g. if we want to count the lengths of a measure to check if it's full, we
can't be 100% certain that the length will be equal to 1, even if the measure
is full. The typical way to handle this is to define a tolerance, and in case of
comparison the values just need to be within the tolerance to be considered
equal. But I don't really like this so much either.

Pros:

- Very easy to implement, to handle, and is intuitive
- Edge-cases like trioles are still handled correctly

Cons:

- Floating point imprecision ruins the mathematical precision and determinism
