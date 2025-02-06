Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Define a 'double_the_length' function that accepts a
reference to a vector and returns a usize. The function
should be able to accept a reference to a vector storing
any data type.

Return the length of the vector (you can acquire this
with the 'len' method) times 2. The function accepts a
reference parameter.

Examples:
double_the_length(&vec![1, 2, 3]))    => 6
double_the_length(&vec![1, 2, 3, 4])) => 8

Does this function need lifetime annotations?
Explain why or why not.

---

Define a `last_two` function that accepts a slice as
a parameter. This function will eventually be invoked
with a &Vec<T>, but we want to use deref coercion to
accept it as a slice. Return a slice as well.

The function should return a slice with the last two
elements of the input.

Examples:
last_two(&vec![1, 2, 3])           => [2, 3]
last_two(&vec![1, 2, 3, 4, 5, 6])  => [5, 6]

Does this function need lifetime annotations?
Explain why or why not.

---

Define a 'first_five' function that accepts two string
slice parameters: 'text' and 'announcement'. The function
should print the value of 'announcement' and return a
slice of the first 5 bytes of 'text'.

Example:
first_five("refrigerator", "Hello") => "refri"

Does this function need lifetime annotations?
Explain why or why not.

--

Define a `find_string_that_has_content` function that
accepts three string slice parameters: `first`,
`second`, and `target`.

Either the first or second string will hold the `target`
substring. Return the string that contains the `target`
content. You can use the `contains` method on a string
to check if holds a substring.

Example:
find_string_that_has_content("programming", "dining", "gram")
=> "programming"

Does this function need lifetime annotations?
Explain why or why not.
*/

```
