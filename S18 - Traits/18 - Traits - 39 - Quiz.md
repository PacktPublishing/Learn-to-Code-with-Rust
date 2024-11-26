What is a trait?

A procedure of sequential steps.
A container for related data.
A data type representing a set of finite values.
! A contract that requires that a type support a behavior.

---

What is true about any type that implements the Eatable trait?

```rust
trait Eatable {
    fn eat(&mut self);

    fn chew(&self) {
        println!("Chewing the food");
    }
}

```

The type must implement both an `eat` method and a `chew` method.

The type must implement a `chew` method but does not have to implement an `eat` method.

! The type must implement an `eat` method but does not have to implement a `chew` method.

The type does not have to implement either an `eat` or `chew` method because both have default implementations.

---

Will this code compile?

```rust
struct Sushi {
    fish: String,
    pieces: u32,
}

fn main() {
    let salmon = Sushi {
        fish: "Salmon".to_string(),
        pieces: 6,
    };

    println!("{}", salmon);
}

```

Yes.
! No, the Sushi struct does not implement the Display trait.
No, the Sushi struct does not implement the Debug trait.
No, the Sushi struct does not implement the Print trait.

---

How do you declare an associated constant in a trait?

! `const NAME: Type = Value;`
`static NAME: Type = Value;`
`let NAME: Type = Value;`
`fn NAME() -> Type;`

---

Consider the following code. What is true?

```rust
trait B {}
trait A: B {}
```

A is a supertrait of B.
B is a subtrait of A.
! A is a subtrait of B.
The code is invalid.

---

What term do we use to describe a method whose purpose it is to retrieve a value from the type?

Supertrait
Trait bound
Setter
! Getter

---

Why does Rust include both a `PartialEq` and `Eq` trait?

Incompetent developer team.
The two are aliases for each other.
There are types like `f64` that support `Eq` but not `PartialEq`.
! There are types like `f64` that support `PartialEq` but not `Eq`.

---

What do we need to add for this code to compile?

```rust
fn print_in_two_formats<T>(output: T) {
    println!("{}", output);
    println!("{:?}", output)
}
```

Use either trait bound syntax to promise that `T` will implement the `Display` trait.

Use either trait bound syntax to promise that `T` will implement the `Debug` trait.

! Use either trait bound syntax to promise that `T` will implement both the `Display` and `Debug` traits.

Use either trait bound syntax to promise that `T` will implement either the `Display` or `Debug` traits.

---

What does the `Clone` trait require?

The implementation of a `Copy` supertrait
A `copy` method
! A `clone` method
A `duplicate` method

---

What does the `where` clause accomplish?

Enable a function to accept multiple generics

Mark a module item as public so that others can find its location

! Declare trait requirements for generics separately from their declaration in angle brackets

Locate the standard library submodule where a name is located

---

TRUE or FALSE: The following code will compile.

```rust
use std::fmt::Display;

struct Latte<T> {
    shop: T,
}

impl<T: Display> Latte<T> {
    fn info(&self) {
        println!("The coffee shop is {self:?}");
    }
}

fn main() {}


```

True
! False
