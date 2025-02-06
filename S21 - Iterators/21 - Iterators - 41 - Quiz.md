What adapter method transforms an iterator to provide the index position of each element?

`map`
`take`
`position`
! `enumerate`
`index`

---

What method creates an iterator that takes ownership over the original collection's elements?

! `into_iter`
`iter`
`iter_mut`
`borrow`

---

What is the difference between the `fold` and `reduce` methods?

There is no difference between the two methods. They are aliases for another.
The `fold` method only accepts a closure. It automatically provides the first iterator value as the starting accumulator value.
! The `reduce` method only accepts a closure. It automatically provides the first iterator value as the starting accumulator value.
The `fold` method and the `reduce` methods accept their arguments in opposite order.

---

What method aggregates the nested values inside all of the iterator's collections into a one-dimensional iterator?

! `flatten`
`collect`
`level`
`crush`

---

What method on the collection creates an iterator that accomplishes the equivalent of this `for` code?

```rust
for value in &collection {

}
```

`into_iter`
! `iter`
`iter_mut`
`borrow`

---

What does the `find` method return?

The first iterator element that matches the closure's condition.

! An `Option` potentially containing the first iterator element that matches the closure's condition.

A `Result` potentially containing the first iterator element that matches the closure's condition.

An iterator containing all the iterator elements that satisfy the closure's condition.

---

What method can we use to check whether every element in an iterator satisfies a condition?

`every`
`any`
`each`
! `all`

---

What method creates an iterator of mutable references?

`into_iter`
`iter`
! `iter_mut`
`borrow`

---

What symbols should you provide between the `cargo run` command and the command line arguments?

`$$`
`-`
! `--`
`?`

---

What function returns an iterator of the command line arguments?

`fs::args`
! `env::args`
`env::arguments`
`io::args`

---

What trait do arrays implement?

`Iterator`
! `IntoIterator`
Both `Iterator` and `IntoIterator`
Neither `Iterator` nor `IntoIterator`

---

What method reverses the order of elements in an iterator?

`reverse`
`flip`
`invert`
! `rev`

---

What does the `read_dir` function return?

A `DirEntry` struct
! A `Result` storing a `ReadDir` struct in the `Ok` variant.
A `Option` storing a `ReadDir` struct in the `Some` variant
A `Result` storing a `DirEntry` struct in the `Ok` variant.

---

What method exhausts an iterator and gathers its yielded elements into a collection type?

! `collect`
`consume`
`get`
`gather`

---

What must the `filter_map` method closure return?

! An `Option` enum
A `Result` enum
A Boolean
A string

---
