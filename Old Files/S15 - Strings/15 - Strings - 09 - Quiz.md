What method concatenates a single character to the end of a String?

`push_str`
`push_char`
! `push`
`add`

---

What method swaps out all occurrences of one character sequence with another?

`replace_all`
`change`
`swap`
! `replace`

---

What's the most flexible function parameter type below?

! `&str`
`String`
`&String`

---

What method removes whitespace at the beginning and end of a String?

`replace`
! `trim`
`clean`
`strip`

---

What's wrong with the following code?

```rust
fn main() {
    let airline1 = "United";
    let airline2 = String::from("American");
    let itinerary = format("{} {}", airline1, airline2);
}
```

The `format` macro cannot accept a `String` argument, only a string slice
The types of the strings passed into the `format` macro must be of a consistent type.
There is one too many arguments passed to the `format` macro.
! The `format!` macro is missing an explanation mark.
There is nothing wrong with the code.

---

TRUE or FALSE: &str and &String are the same type.

True
! False

---

What method converts a String to one with all capital letters?

! to_uppercase
toUpperCase
uppercase
upper
capitalize

---

The `trim` method returns...

A new String
! A string slice

---

What is wrong with the following code?

```rust
fn main() {
    let airline = "United";
    let first_initial = &airline[0];
}
```

The borrowing syntax only works with a `String`, not a string slice (`&str`).
! Square bracket syntax requires a range to extract a byte sequence. An individual index position will not work.
It needs to remove the borrow operator.

---

Will the following code compile?

```rust
fn main() {
    let airline = "United Airlines Flight";
    let segments = airline.split(" ").collect();
}

```

Yes
! No
