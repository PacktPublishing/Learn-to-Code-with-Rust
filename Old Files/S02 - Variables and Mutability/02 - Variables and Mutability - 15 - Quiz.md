What keyword declares a variable?

`var
`declare`
`const`
!`let`

---

A variable is `____` by default?

Mutable
! Immutable

---

What characters interpolate a dynamic value into a string?

`<>`
! `{}`
`:?`
`""`

---

Rust counts positions starting from

-1
! 0
1
2

---

What keyword declares a variable as mutable?

None. A variable is mutable by default.
`mute`
! `mut`
`mutable`
`const`

---

What character tells the compiler to ignore an unused variable?

`#`
`!`
`?`
! `_`

---

What will be the result of this code?

```rust
fn main() {
    let latte = "Latte";
    let cappuccino = "Cappuccino";
    println!("I like {1} and {2}", latte, cappuccino);
}
```

"I like Latte and Cappuccino"
"I like Cappuccino and Latte"
"I like latte and cappuccino"
! The code will not compile

---

What is the correct syntax for applying a compiler directive to the following line?

`[allow(unused_variables)]`
! `#[allow(unused_variables)]
`#[!allow(unused_variables)]
`!#[allow(unused_variables)]
`#(allow[unused_variables])

---

What keyword declares a constant in the program?

`let`
! `const`
`var`
`final`

---

How do I fix the following code?

```rust
fn main() {
  let city = "Metropolis";
  city = "Gotham";
}
```

The code doesn't need a fix. It will compile and run fine.
Add a `mut` keyword before the `let` keyword in the variable declaration.
! Add a `mut` keyword after the `let` keyword in the variable declaration.
Add a `mut` keyword before the `city` variable when assigning the new value (`"Gotham"`) to it.

---

What word means "the region of code where a name is valid"?

Function
Source code
Compilation
! Scope

---

What's wrong with the following code?

```rust
fn main() {
    let mut genre = "Romance";
    genre = 52;
}

```

There is nothing wrong with the code.
The `mut` keyword is not required.
There should't be a semicolon after the assignment of 52.
! The code is trying to assign an integer to a variable expecting a string.

---

Which characters create a block?

Pair of parentheses (`()`)
Pair of square brackets (`[]`)
Two forward slashes (`//`)
! Opening and closing braces (`{}`)

---

What keyword declares an alias/nickname for an existing type?

`redeclare`
! `type`
`alias`
`rename`

---

What will be output when this code runs?

```rust
fn main() {
  let city = "Huntington";
  println!("{city}");
  let city = "Plainview";
}
```

! Huntington
Plainview
city
{city}
