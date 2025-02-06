How does Rust identify a library crate?

The presence of a `src/main.rs` file
! The presence of a `src/lib.rs` file
The presence of a `src/library.rs` file
The presence of a `src/crate.rs` file

---

What keyword makes a name accessible outside of its module?

`expose`
`public`
! `pub`
`export

---

What symbol imports all of the names from a module?

`@`
`#`
`$`
! `*`

---

How does Rust identify a binary crate?

! The presence of a `src/main.rs` file
The presence of a `src/lib.rs` file
The presence of a `src/library.rs` file
The presence of a `src/binary.rs` file

---

What keyword specifies the top-level of the crate root?

`root`
! `crate`
`top`
`absolute`

---

TRUE or FALSE: Marking a struct as public with `pub` allows it to be instantiated outside of its module.

True
! False

---

What keyword assigns an alias to an import?

`alias`
`rename`
`nickname`
! `as

---

Say we declare `mod danger` in our crate root in `src/main.rs`. Which of the following options is a valid way to declare this `danger` module?

Create a `src/danger/danger.rs` file
Create a `src/mod.rs` file.
! Create a `src/danger.rs` file
Create a a `src/mod/danger.rs` file

---

What keyword navigates upwards into the parent module?

! `super`
`root`
`crate`
previous
up

---

Say we declare `mod airplane` in our crate root in `src/main.rs`. Which of the following options is a valid way to declare this `airplane` module?

Create a `src/main/airplane.rs` file
Create a `src/airplane/airplane.rs` file
Create a `src/mod/airplane.rs` file
! Create a a `src/airplane/mod.rs` file

---

TRUE or FALSE: A package can have either a library crate or a binary crate but not both.

True
! False

---

What names will be available to use within this file?

```rust
use movies::production::{Director as MainGuy,Actor};
use music::production::{self, Engineer};
```

Director, Actor, and Engineer
Director, Actor, production, and Engineer
Director, MainGuy, Actor, and Engineer
Director, MainGuy, Actor, production, and Engineer
! MainGuy, Actor, production, and Engineer
