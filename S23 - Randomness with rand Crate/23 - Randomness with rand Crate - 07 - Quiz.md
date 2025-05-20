What crate in the Rust ecosystem helps with randomness?

`random`
! `rand`
`clap`
`rstest`

---

The `rand` crate is a...?

! Library crate
Binary crate

---

What method can help you model an event with a 15% chance of happening?

`odds`
`random`
! `random_bool`
`percent_chance`

---

What's wrong with the following code?

```rust
use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    let generator = rng();
    let mut teas = ["Earl Grey", "Chamomile", "Peach Oolong"];
    teas.shuffle(&mut generator);
    println!("{:#?}", teas);
}
```

We cannot shuffle an array. We can only shuffle a vector.
The `shuffle` method expects mutable ownership of the generator, not a mutable reference.
The `SliceRandom` trait being brought into scope is unnecessary.
! The `generator` variable must be made mutable.

---

How many possible values can this range produce?

```rust
use rand::{rng, Rng};

fn main() {
    let mut generator = rng();

    let random_number: i32 = generator.random_range(1..75);
    println!("{}", random_number);
}

```

None
73
! 74
75
76

---

What trait needs to be in scope to use a method on the random number generator?

! `Rng`
`Range`
`Ranger`
`Random`

---

What method can generate a random floating-point value?

`random_float`
`random_number`
`rand_float`
! `random`

---

Which function returns the thread-local random number generator?

`mr_rng`
`random_gen`
`random_ratio`
`rand_num_gen`
! `rng`

---

What’s the difference between using `random_range(1..5)` and `random_range(1..=5)`?

The first option will not compile.
The second option will not compile.
The first option has the potential to yield one more value.
! The second option has the potential to yield one more value.
The two are equal.

---

FUN FACT: The unicorn is the national animal of Scotland.

! Thanks! That was totally random!
There's no way that's true!
`random_range(2..=6)`
I have no sense of humor!
