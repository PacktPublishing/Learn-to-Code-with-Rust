What are the two variants in a `Result` enum?

Some and None
Some and Err
Ok and None
! Ok and Err

---

TRUE or FALSE: Ownership will transfer from `a` to `b` in the following example.

```rust
fn main() {
    let a = Ok::<u32, &str>(5);
    let b = a;
}
```

True
! False

---

What are the two variants in a `Option`?

! Some and None
Some and Err
Ok and None
Ok and Err

---

TRUE or FALSE: Ownership will transfer from `a` to `b` in the following example.

```rust
fn main() {
    let a = Some("thing".to_string());
    let b = a;
    println!("{:?}", a);
}
```

True
! False

---

What method pulls out the value inside a `Some` variant of an `Option`?

extract
get
! unwrap
dig

---

TRUE or FALSE: The following code will compile.

```rust
fn main() {
    let video_rental_in_stock = Some(false);

    let value = match video_rental_in_stock {
        Some(false) => false,
        None => false,
    };
}

```

True
! False

---

`Option` and `Result` enums implement...

Both the `Display` trait and the `Debug` struct
Only the `Display` trait
! Only the `Debug` trait
Neither the `Display` trait nor the `Debug` trait

---

Why will the the following code not compile?

```rust
fn main() {
    let unknown: Result<&str> = Ok("Success");
}
```

`Ok` is an `Option` variant, not a `Result` variant
The type of the generic should be a `&String`, not a `&str`.
The `Ok` variant must have a `Result` prefix.
! The `Result` type declaration requires a second generic.

---

What method provides a fallback value in case we cannot extract the value from a `Ok` variant?

default
! unwrap_or
fallback
backup

---

Why will the following code not compile?

```rust
fn main() {
    let unknown: Result<String, bool> = Ok(false);
}

```

! We need to reverse the order of the generics to match the right-hand declaration.
The `Result` type declaration requires only one generic
`Ok` must have a `Result` prefix.
`Ok` is an `Option` variant, not a `Result` variant.

---

What term do we use to refer to the collection of names that are automatically available in every Rust program?

The essentials
The registry
The toolbox
! The prelude
