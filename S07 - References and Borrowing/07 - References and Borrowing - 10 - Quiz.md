What does Rust permit for a given value at the same time time?

Any number of mutable references
! Any number of immutable references
One mutable reference and one immutable reference
A maximum of two mutable references

---

Who is the owner of the `String` when the function concludes?

```rust
fn main() {
    let candy = String::from("M&M's");
    let treat = candy;
}
```

The `candy` variable
! The `treat` variable

---

What is a dangling reference?

A reference that points to another reference
An immutable reference that is converted to a mutable one
! A reference to a memory address where data no longer exists
A reference to a data type stored on the heap

---

Who is the owner of the `String` when the function concludes?

```rust
fn main() {
    let candy = String::from("M&M's");
    let treat = &candy;
}
```

! The candy variable
The treat variable

---

Will this code compile?

```rust
fn main() {
    let candy = String::from("M&M's");
    let some_ref = &candy;
    let another_ref = &candy;
    println!("{some_ref} and {another_ref}");
}

```

! Yes
No

---

What keyword should you add before a parameter name to allow the function to modify its content?

`&`
`*`
! `mut`
`mute`

---

Who is the owner of the String when the function concludes?

```rust
fn main() {
    let candy = String::from("M&M's");
    let treat = candy;
    let diet_plan = [treat];
}
```

The candy variable
The treat variable
! The diet_plan array

---

Will this code compile?

```rust
fn main() {
    let mut candy = String::from("M&M's");
    let some_ref = &mut candy;
    let another_ref = &candy;
    println!("{some_ref} and {another_ref}");
}
```

Yes
! No

---

Will this code compile?

```rust
fn main() {
    let candy = String::from("M&M's");
    let some_ref = &mut candy;
    println!("{some_ref}");
}

```

Yes
! No.

The original candy variable is not marked as mutable with `mut`, so we cannot create a mutable reference from it.

---

What operator follows a reference to the value at its memory address?

`&`
`!`
`{}`
! `*`

---

TRUE or FALSE: There is no technical difference between the two functions below.

```rust
fn make_empty(mut content: String) {
    content.clear()
}

fn make_empty(content: &mut String) {
    content.clear()
}
```

True
! False. 

The first function takes ownership of the String while the second borrows ownership of the String.
