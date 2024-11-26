What format should enum variants be written in?

snake_case
upperCamelCase
! PascalCase
kebab-case

---

TRUE or FALSE: All variants within a single enum must use the same variant type (i.e., struct variant, tuple variant).

True
! False

---

What is wrong with the following code?

```rust
enum Vehicle {
    Car(String),
    Bus,
    Train,
}

fn main() {
    let toyota = String::from("Toyota");
    let vehicle = Vehicle::Car(toyota);
    println!("{toyota}");
}
```

There is nothing wrong with the code.

There is a mismatch in types. The Car variant expects a string slice, not a String.

The Car variant requires curly braces ( {} ) instead of parentheses to create.

! The toyota variable is no longer the owner of the String, so we cannot print it afterwards.

We need to add an extra colon after the Vehicle enum to access the Car variant.

---

What keyword declares a block for enum method definitions?

define
! impl
implementation
methods
imp

---

When using the `if let` construct, where must you write the enum variant to match against?

! The left-hand side of the equal sign
The right-hand side of the equal sign
Either side of the equal sign

---

What keyword compares a value against all possible enum variants?

compare
eq
if let
switch
! match

---

What character acts as a catch-all pattern in a `match` expression?

`!`
`?`
`^`
`&&`
! `_`

---

What is the syntax to derive the `Debug` implementation for an enum?

`#(derive[Debug])`
! `#[derive(Debug)]`
`#(derive(Debug))`
`$[derive(Debug)]`
