Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Bring the Debug trait, the Display trait, and the
Formatter struct into the current file scope. They are
all found inside the `fmt` submodule in the standard
library.

---

Define a Drinkable trait with 3 required methods:
- A `consume` method that accepts a mutable reference
- A `get_data` method that accepts an immutable reference
  and returns a String. This will serve as a getter
- A `stats` method that accepts an immutable reference.
  It should output the return value of the `get_data`
  method in Display format

---

Define a Milk enum with 3 variants: Whole, Oat, and
Almond. Derive Rust's default Debug implementation for
the Milk enum.

---

Define a Coffee struct with one generic `T`.
It should hold 3 fields:
- A `kind` field of type T
- A `milk` field of type Milk
- An `ounces` field of type u32

Define a `new` constructor function for Coffee that
returns a new Coffee instance. The function should be
available for any generic type T.

---

Define a manual Debug trait implementation for a
Coffee struct.

Use the formatter struct and its `debug_struct` and
`field` methods to output all three fields and their
respective values. You'll need to place a Debug trait
constraint on generic type T to enable this.

---

Implement the Drinkable trait for a Coffee struct of
any type T. Place a Display trait constraint on generic
type T.
- The `consume` method should decrement the `ounces`
  field to 0.

- The `get_data` method should return the String
  "A delicious __ ounce ___", where the two dynamic
  values come from the `ounces` and `kind` fields

---

Define a Soda struct with 4 fields:
- A `calories` field set to a u32
- A `price` field set to a f64
- A `flavor` field set to a String
- A `percentage` field set to a u32

Derive Rust's default Debug implementation for the
Soda struct.

Define a `new` constructor function that returns a
Soda instance. The `percentage` field should initialize
with a hardcoded value of 100; the other fields should
be set by parameters.

---

Implement the Drinkable trait for the Soda struct.
- The `consume` method should decrement the
  `percentage` field to 0.

- The `get_data` method should return the String
  "Flavor: __, Calories: ___", where the two dynamic
  values come from the `flavor` and `calories` fields.

---

Implement the Display trait for the Soda struct. It
should use the `write!` macro to output the text
"** ___ Soda **" where the dynamic value comes from
the `flavor` field.

---

Implement the Clone trait for the Soda struct. The
new Soda should copy over the values from the original
Soda, including making a clone of the `flavor` field.

---
Implement the PartialEq trait for the Soda struct.
Two Sodas should be considered equal if they have the
same price.

Implement the Eq subtrait for the Soda struct as well.

---

In the `main` function,
- Create a Coffee instance with the `new` constructor
  function. Use a string slice for the `kind` parameter.
  Choose any Milk and number of ounces. Assign it to a
  `latte` variable.

- Output the `latte` variable in Debug format.

- Invoke the `consume` method.

- Output the `latte` variable in Debug format again to
  observe the expected changes.

---

- Create a Coffee instance with the `new` constructor
  function. Use a String for the `kind` parameter.
  Choose any Milk and number of ounces. Assign it to
  a `cappuccino` variable.

- Invoke the `get_data` method on `cappuccino` and
  output the result.

---

- Create a Soda instance with the `new` constructor
  function. Choose any value for calories and price.
  Use a flavor of "Cherry". Assign to a `pepsi` variable.

- Output the `pepsi` variable in Display format. You
  should see "** Cherry Soda **".

---

- Use the `clone` method to create a duplicate of the
 `pepsi` variable. Assign it to a `coke` variable.

- Compare the equality of the `pepsi` and `coke`
  variables. Output the Boolean result.

- Invoke the `consume` method on the `coke` variable.

- Output the `coke` variable in Debug format.
*/

```
