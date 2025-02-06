What is the correct subtrait hierarchy (from top to bottom)?

`FnOnce`, `Fn`, `FnMut`
`FnMut`, `Fn`, `FnOnce`
`FnMut`, `FnOnce`, `Fn`
! `FnOnce`, `FnMut`, `Fn`

---

True or False: You can pass in an `FnMut` closure to a parameter that requires an `Fn` closure.

True
! False

---

What keyword forces a closure to take ownership of its values?

`change`
! `move`
`own`
`ref`

---

How can we clean up this line of code without altering its functionality?

```rust
let sum = |a: u8, b: u8| { a + b };
```

Provide an explicit closure type annotation before the equal sign.
Provide an explicit return type with the arrow syntax.
Remove the semicolon at the end.
! Remove the curly braces

---

True or False: You can pass in an `Fn` closure to a parameter that requires an `FnMut` closure.

! True
False

---

What will the `test` method return in the following invocation?

```rust
fn test<F>(f: F) -> String
where
    F: FnOnce() -> String,
{
    f().to_uppercase()
}

fn main() {
    let movie = String::from("Die Hard");
    test(|| movie);
}

```

Die Hard
! DIE HARD
die hard
The program is not valid.

---

Which closure type cannot be invoked multiple times?

! `FnOnce`
`FnMut`
`Fn`
All closure types can be invoked multiple times.

---

What's wrong with the following code?

```rust
fn test<F>(f: F)
where
    F: FnMut(),
{
    f();
}
```

! The `f` parameter must be marked as mutable.
We cannot have a parameter and a generic be the same name in different casings.
The trait bound to the `F` generic should be `Fn`.
There's nothing wrong with the code.

---

What entity can capture values from an outer scope?

Only functions
! Only closures
Both functions and closures
Neither functions nor closures

---

What will be the inferred type of the `add_song` closure below?

```rust
use std::collections::HashMap;

fn main() {
    let mut playlist = HashMap::new();
    let add_song = |song: String, genre: String| {
        playlist.insert(song, genre);
    };
}
```

`FnOnce`
! `FnMut`
`Fn`
