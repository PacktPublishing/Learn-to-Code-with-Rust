Which macro creates a new vector?

`v!`
! `vec!`
`vect!`
`vector!`

---

What does the `get` method on a vector return?

An `Option` enum where the `Some` variant stores the vector's value at a given index position
! An `Option` enum where the `Some` variant stores a reference to the vector's value at a given index position
A `Result` enum where the `Ok` variant stores the vector's value at a given index position
A `Result` enum where the `Ok` variant stores a reference to the vector's value at a given index position

---

What method adds an element to the vector at a given index position?

add
push
! insert
shift

---

TRUE or FALSE: This code will compile.

```rust
fn main() {
    let cities = vec![
        String::from("Nashville"),
        String::from("Cleveland"),
        String::from("Cincinnati"),
    ];

    let destination = cities[1];
    println!("{destination}");
}

```

True
! False. 

We cannot move ownership of one element out of the vector.

---

What `Vec` constructor function returns an empty vector with a specific capacity?

new
max_size
capacity
! with_capacity

---

TRUE or FALSE: This code will compile.

```rust
fn main() {
    let cities = vec![
        String::from("Nashville"),
        String::from("Cleveland"),
        String::from("Cincinnati"),
    ];

    let destination = &cities[1];
    println!("{destination}");
}

```

! True
False

---

What method appends an element to the end of the vector?

`add`
`pop`
! `push`
`shift`

---

TRUE or FALSE: This code will compile.

```rust
fn main() {
    let cities = vec![
        String::from("Nashville"),
        String::from("Cleveland"),
        String::from("Cincinnati"),
    ];

    let target = &mut cities[1];
    let destination = &cities[1];
    println!("{destination}");
}

```

True
! False

---

What does the `pop` method return?

A `Result` with potentially the first vector element
An `Option` with potentially the first vector element
A `Result` with potentially the last vector element
! An `Option` with potentially the last vector element

---

TRUE or FALSE: This code will compile.

```rust
fn main() {
    let mut cities = vec![
        String::from("Nashville"),
        String::from("Cleveland"),
        String::from("Cincinnati"),
    ];

    let target = &mut cities[1];
    let destination = &cities[1];
    println!("{target}");
}
```

True
! False. 

We cannot have more than one mutable reference to a value at a time.
