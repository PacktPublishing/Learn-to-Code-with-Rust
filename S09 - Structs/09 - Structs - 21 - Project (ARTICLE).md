Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Define a Flight struct with the following fields:
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)

Derive a Debug trait implementation for the Flight struct.

Define a `new` constructor function that returns a new
instance of a Flight.

Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field.

Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value.

Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination).

Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.

Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/
```
