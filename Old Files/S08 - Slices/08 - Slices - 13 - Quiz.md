What is a slice?

When partial ownership of a value is moved to a new owner
A function that splits a string into substrings based on a delimiter
! A reference to a sequence of elements in a collection
A reference that points to deallocated memory

---

What will this program output?

```rust
fn main() {
    let city = String::from("New Orleans");
    println!("{}", &city[5..9]);
}
```

`Orle`
`Orlea`
! `rlea`
`rlean`

---

What does the `len` method return on a string?

A count of its characters
A count of its references
! A count of its bytes

---

What will this code do?

```rust
fn main() {
    let mut coffees = [
        String::from("Flat White"),
        String::from("Latte"),
        String::from("Cold Brew"),
        String::from("Matcha Tea"),
        String::from("Cortado"),
    ];

    let lunch = &mut coffees[2..];
    lunch[1] = String::from("Frappuccino");
}

```

Raise a runtime panic.
Replace the "Cold Brew" string with "Frappuccino"
! Replace the "Matcha Tea" string with "Frappuccino"
Replace the "Cortado" string with "Frappuccino"

---

Which two syntax options below are equal?

`&text[0..5] and &text[0...5]`
! `&text[0..5] and &text[..5]`
`&text[0..5] and &text[..]`
`&text[0..5] and &text[0..]`

---

What are the types of variables `a` and `b` in this example?

```rust
fn main() {
    let city = String::from("Dallas");

    let a = &city;
    let b = &city[..];
}
```

`&str` and `&str`
`&String` and `&String`
`&str` and `&String`
! `&String` and `&str`

---

Say we define a function parameter set to the type `&str`. What kind of arguments can the function accept?

Only string slices (`&str`)
Only String references (`&String`)
! Both String slices (`&str`) and String references (`&String`)
Neither string slices (`&str`) nor String references (`&String`)

---

What will this program output?

```rust
fn main() {
    let city = String::from("New Orleans");
    println!("{}", &city[..5]);
}

```

`New`
`New `
! `New O`
`leans`

---

Which two syntax options below are equal?

`&text[8..13] and &text[8..=13]`
`&text[8..] and &text[0..8]`
`&text[8..] and &text[8..=]`
! `&text[8..] and &text[8..13]`

---
