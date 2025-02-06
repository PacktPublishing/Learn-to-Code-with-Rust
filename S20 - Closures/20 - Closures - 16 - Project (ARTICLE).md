Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Define a `SupermarketItem` struct with a `name` field
set to a String and a `price` field set to a f64.
Derive a Debug trait implementation.

Define a `ShoppingCart` struct with an `items` field
set to a vector of `SupermarketItem` structs.
Derive a Debug trait implementation.

---

Define a `traverse_items` method on the `ShoppingCart`.
It should accept a mutable reference to the instance
as well as an `operation` parameter that will be a
closure. Use a trait bound to mandate the closure implement
the `FnMut` trait. In addition, the closure should accept
a mutable reference to a `SupermarketItem`.

In the body of the `traverse_items` method, perform an
iteration over all `SupermarketItem`s in the `items` vector
using a `while` loop. For each vector element, invoke the
`operation` closure and pass in a mutable reference to the
`SupermarketItem` to the closure.

---

Define a `checkout` method on the `ShoppingCart`. It should
take ownership of the instance. It should also define an
`operation` parameter that will be a closure. Use a trait
bound to mandate the closure implement the `FnOnce` trait.
In addition, the closure should accept a single parameter of
a ShoppingCart.

In the body of the `checkout` method, invoke the `operation`
parameter and pass in the `ShoppingCart` instance.

---

In the body of `main`, create a `ShoppingCart` instance.
Populate its `items` vector with two `SupermarketItem`
instances. For the names of the items, provide capitalized
strings. Here's some sample data:
- { name: "APPLE", price: 3.99 }
- { name: "BANANA", price: 2.99 }

Invoke the `traverse_items` method and pass in a closure.
The closure will accept a mutable reference to each
`SupermarketItem` struct. Decrease the item's price by 15%
(you can multiply the price by 0.85 to accomplish this).

Invoke the `traverse_items` method again and pass in another
closure. This time around, take the `SupermarketItem`'s name,
lowercase it, and overwrite the existing `name` field value.

---

Declare a `total_price` variable initialized to 0.0.
Invoke the `checkout` method and pass in a closure.
The closure should receive mutable ownership of the
`ShoppingCart` struct. In the body of the closure, output
the cart in Debug format.

Then, call the `traverse_items` closure IN THIS closure.
Pass in a closure to `traverse_items`. The nested closure
should increment the `total_price` value so that it gathers
the cumulative sum of all of the `SupermarketItem`'s prices.

Print out the final value of the `total_price` variable
with a precision of 2 decimal points and a dollar sign
in front.
*/