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

Well, all this music theory and we must somehow implement it for this library.
Let's go step by step from the ground up and try to be as general as possible
to include as many different configurations as possible.

## Note lengths

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

### Idea 1: Defining a highest level of subdivision

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

### Idea 2: Two integers for subdivision level and count

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

Note from the future: This is extremely similar to the floating point format
described below, it uses the same ideas even. Basically, with this approach I
basically reinvented floating point numbers with less functionality and less
optimization.

Pros:

- Crazily deep subdivision possible (up until ~ 1 / (1.7 \* 10^38) if quarter is 1/4)
- Also goes up very far (256 \* 3.4\*10^38 whole notes possible)

Cons:

- Wide range of redundant and unnecessary values
- Still no trioles

### Idea 3: Using floats

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

In the integer representation, we could still represent trioles if we accept
imprecision. A quarter-triole would be (2/3)*(1/4) = 0.16667 for floats, with
the first method we'd need 42.67 or 43 256th notes, and loose about 0.33 of a
256th of precision. The second approach could go for 170.67 1024th notes, which
much more precise (though the implementation would need to choose the most
optimal set of values above other redundant equivalents).

The flost would be able to store 0.166666656, which would loose about
0.000000011. The second approach above would store 0.166992188 and lose about
0.166992188. The second approach is similar to a float with 8-bit mantissa and
8-bit exponent. The float is larger (32 bits) and is therefore more precise.

Floats can actually also perfectly represent subdivisions of two (0.5, 0.25,
...), so this is the "official" version of the second idea.

Pros:

- Very easy to implement, to handle, and is intuitive
- Edge-cases like trioles are still handled correctly

Cons:

- Floating point imprecision ruins the mathematical precision and determinism

### The triole problem

None of the ideas above seem to really satisfy trioles or n-toles fully. The
integer approaches satisfy everything except trioles. The float approach could
work but is imprecise.

In music theory, trioles are also kind of a special case. Triole notes cannot
appear individually, they must always be in a bundle of three notes. The note
length of these is always the same. A quarter triole consists of three
"quarters", whose length is so that all three together sum up to a half note.
Essentially, a quarter triole divides a half note into 3 different notes.

n-toles will work the same way, just that the note length one level above the
denoted lengths will be divided into n notes.

It would be fancy if my implementation could handle individual "triole notes",
but I could also try to implement it similarly to maintain the perfect
subdivision. But this might be difficult considering my architecture.

An n-tole will contain n-notes all with the same length, and the total length
would be one subdivision above the note's lengths. The issue is that all notes
are stored in a vector, and we can't just put our n-tole wrapper inbetween
there. It also makes no sense to store this info inside the notes.

It seems like we're approaching our old implementation with the trioles. In the
old code, there's just a simple flag telling us if the note is part of a triole.
There's no checking if those "triole notes" are part of a bundle, or if they're
just individual, allowing indivual triole notes (which is fancy). I just didn't
implement the actual calculations for those.

The issue with individual triole notes is that they cannot be added together
to regular notes without losing precision for length. We always need 3 triole
notes (or n n-tole notes) to simplify the length into a subdividable length.
It actually makes no sense to place e.g. just 2 triole notes in a measure
because it will never perfectly fit without a third triole note.

So let's define that n-tole notes must always have n occurences in the same
measure to be valid. If it isn't like that, the program shall return an error.

### Final design

Let's arrive at the final design of how note lengths should be represented.
I'll go simple and choose the first idea because it is the easiest to work with.

The smallest subdivsision shall be a 2^16th, which is a 65536th. This is
overkill but it ensures we will basically never run out of subdivisions. The
length field itself should be at least a u32, but larger fields can be used if
desired.

A whole note length would then be represented as 2^16 or 65536. A half is 2^15
and a quarter 2^14. We can also dot the notes many times, because the value is
divisible by 2 a lot of times.

Operations like addition or subtractions are trivial since this is a simple
number fields, and I won't elaborate on those further (except for n-tole cases).

The length shall also contain an n-tole field. The formula n = (2x + 1) shall
apply where x is the value of the field. A value of x = 0 will represent no
n-tole or just a simple note length, x = 1 is a triole (n = 3), x = 2 a pentole,
and so on.

When precisely counting lengths, there shall be n placed notes with the same
n-tole length to simplify to a subdividable value. When converting to duration,
floating point representation is sufficient.

## Time signatures

The time signature is a struct containing the following info:

### Measure length

The measure length is a note length signifying how long the measure should be.
This results from calculating the "fraction" of the typical notation, e.g. a
4/4 signature will simplify to 1, a 2/4 to 0.5, 3/4 to 0.75, etc...

We don't store the nominator or denominator, just the result.

### Beats

The beats are the positions inside the measure where notes are emphasized.
Let's define a single beat to be represented by a note length, and by an
"emphasis level". All beats are stored sequentially (e.g. in a Vector) and are
required to add together to the measure length.

The emphasis level will be a positive floating point value. If the value is
equal to 1, the beat will be "normal". A value greater than 1 will emphasize
the beat with the specified level, while a value smaller than 1 will weaken
the beat. As far as I know, "weakening" a beat does not exist in music
theory, but let's include it consistency.

### Examples for time signatures

Below are some examples of how the music time signatures are represented as
in-code time signatures. I've always set the strong emphasis to be 1.1 and
weak emphasis to be 1.01, but that's neither required nor some standard I'm
abiding, I arbitrarily chose those values.

#### 4/4 time

Measure length: 4/4 = 1.0

Beats:

- Quarter; 1.1   // Strong emphasis
- Quarter; 1.0   // No emphasis
- Quarter; 1.01  // Weak emphasis
- Quarter; 1.0   // No emphasis

#### 2/4 time

Measure length: 2/4 = 0.5

Beats:

- Quarter; 1.1   // Strong emphasis
- Quarter; 1.0   // No emphasis

#### 3/4 time

Measure length: 3/4 = 0.75

Beats:

- Quarter; 1.1   // Strong emphasis
- Quarter; 1.0   // No emphasis
- Quarter; 1.0   // No emphasis

#### 6/8 time

Measure length: 6/8 = 3/4 = 0.75

Beats:

- Eigth; 1.1   // Strong emphasis
- Eigth; 1.0   // No emphasis
- Eigth; 1.0   // No emphasis
- Eigth; 1.01  // Weak emphasis
- Eigth; 1.0   // No emphasis
- Eigth; 1.0   // No emphasis

#### 5/4 time

Measure length: 5/4 = 1.25

Beats:

- Quarter; 1.1
- Quarter; 1.0
- Quarter; 1.1
- Quarter; 1.0
- Quarter; 1.0

OR

- Quarter; 1.1
- Quarter; 1.0
- Quarter; 1.0
- Quarter; 1.1
- Quarter; 1.0
