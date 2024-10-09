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
