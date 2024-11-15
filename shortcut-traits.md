# Shortcut traits implementation ideas

This is a temporary document for collecting ideas on how to implement shortcut
traits. This will be deleted after implementation is done.

## `impl` on trait (Instrument)

It's possible to do an `impl` on the trait Instrument. This is essentially
exactly what I was looking for.

The only problem is that it won't work so well right now. Because of the dynamic
nature of the Instrument trait, there are some weird errors regarding a "vTable"
and the "ConcreteType". I don't think I could fix these without massively
refactoring how the Instrument trait works, so it is out of the question now.

It will also not be possible to combine multiple shortcuts together.

## Splitting Instrument into multiple traits; default implementations

If I were to split the Instrument trait into multiple traits, e.g. one is
responsible for dynamics, the other for something else, it would be possible
to provide default implementations for a bunch of these. The user can still
override them if wanted, but in a normal use case the user doesn't have to
worry about these traits.

There are two problems: The most obvious one is that this is a breaking change.
I want to keep it unbreaking for now, and that would be rather difficult.

The second problem is that I only have a faint idea of how this even might look
like. I feel like the way I want to do this doesn't actually make sense.

### Deeper analysis

This method works best for the dynamics-shortcut, because the dynamics
calculation can be externalized from the core sample rendering. The major
problem is with the shortcut of returning singular samples instead of the whole
buffer.

Essentially, we let the user choose if they want to operate on and return a
buffer, or on singular samples. If the latter is chosen, we need to
automatically handle those samples ourselves.

The problem is that it's impossible to generalize these two behaviours into one.
One behaviour returns just one sample, the other the whole buffer.

## Using macros

Macros could be useful, but I think that this will turn everything into a mess.
If other options yield diminishing results, this might be the way to go.
