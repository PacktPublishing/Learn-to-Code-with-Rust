Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Define a Tier enum with three variants: Gold, Silver,
Platinum. Derive a Debug implementation for the Tier enum.

Define a Subscription enum with three variants: Free,
Basic, and Premium. Derive a Debug implementation for the
Subscription enum.

The Free variant should have no associated data.

The Basic variant should be a tuple variant with two pieces
of data. The first one should be a f64 (the price per month)
and the second one should be a u32 (the number of months).

The Premium variant should be a struct variant with a 'tier'
field. The tier field should be a Tier enum value.

Define a 'summarize' method on the Subscription enum.

If the Subscription is Free, output the text "You have
limited access to the site".

If the Subscription is Basic, output the text "You have
limited access to the site's premium features for {price}
for {months} months", where {price} amd {months} come from
the tuple variant's associated data.

If the Subscription is Premium, output the text "You have
full access to the site's premium features. Your tier is
{tier:?}"", where {tier} comes from the struct variant's
associated 'tier' field.

In the `main` function, create 3 instances of Subscription,
each one with a different variant. Invoke the `summarize`
method on each instance.
*/

```
