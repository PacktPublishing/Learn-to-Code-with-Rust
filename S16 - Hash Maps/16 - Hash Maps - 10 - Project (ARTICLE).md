Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Bring the HashMap type into the current's file's namespace.

Declare a `sauces_to_meals` HashMap. The keys will be
string slices and the values will be a vector of string
slices. Use the `from` function to populate the HashMap
with 2 key-value pairs:

Key: "Ketchup"
Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]

Key: "Mayonnaise"
Value: Vector of ["Sandwiches", "Burgers", "Coleslaw"]

Use the `insert` method to add the following key-value
pair to the HashMap.

Key: "Mustard"
Value: Vector of ["Hot dog", "Burgers", "Pretzels"]

Use the `remove` method to remove the key-value pair
where "Mayonnaise" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `get` method to retrieve the key-value pair
where "Mustard" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `entry` and `or_insert` methods to add the
following key-value pair:

Key: "Soy Sauce"
Value: Vector of ["Sushi", "Dumplings"]

Finally, print out the final `sauces_to_meals` HashMap.

The final result should be:
{
  "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
  "Soy Sauce": ["Sushi", "Dumplings"],
  "Mustard": ["Hot dog", "Burgers", "Pretzels"]
}
*/

```
