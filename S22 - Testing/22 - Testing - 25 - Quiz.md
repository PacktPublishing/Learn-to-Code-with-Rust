What annotation declares that a function is a test function?

`#[assert]`
`#[fixture]`
! `#[test]`
`#[validate]`

---

What annotation should you add to the `tests` module to exclude its code from the compiled executable?

`#[test]`
`#[configure(test)]`
! `#[cfg(test)]`
`#[exclude(test)]`
`#[include(test)]`

---

What macro validates that two values are equal?

`assert_equal!`
! `assert_eq!`
`assert_same!`
`validate_equal!`

---

What annotation validates that test code creates a panic?

`#[validate_panic]`
`#[should_fail]`
`#[assert_panic]`
! `#[should_panic]`

---

Instead of assertions, what can a test function return to indicate a passing test?

! A `Result` enum's `Ok` variant
A `Result` enum's `Err` variant
An `Option` enum's `Some` variant
An `Option` enum's `None` variant

---

What crate helps automate the creation of fixtures?

`pretty_assertions`
`rand`
! `rstest`
`mockall`

---

What is not ideal about this design?

```rust
struct UserService;

impl UserService {
    fn get_user(&self) {
        println!("Getting user...");
    }
}

struct Database;

impl Database {
    fn query(&self) {
        let service = UserService;
        service.get_user();
        println!("Querying database...");
    }
}
```

The `get_user` method would work better as a plain function.

The two structs should be consolidated into a single one.

! The `Database` struct does not utilize dependency injection so it is coupled to the `UserService`.

There is nothing to improve in this code.

---

What syntax pulls in all names from the outer module into the current module?

`super::all`
! `super::*`
`use::*`
`import::*`

---

What are the 3 phases of the test-driven development (TDD) paradigm?

Plan-Write-Execute
! Red-Green-Refactor
Eat-Sleep-Code
Product-Design-Engineering

---

What trait must the values passed into `assert_eq!` implement?

! `PartialEq`
`PartialOrd`
`Display`
`Clone`
