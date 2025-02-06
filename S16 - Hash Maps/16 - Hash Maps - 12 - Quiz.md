Which statement about hash maps is correct?

Neither keys or values can contain duplicates.
Both keys and values can contain duplicates.
! Keys must be unique, values can contain duplicates.
Keys can contain duplicates, values must be unique.

---

The code below creates a hashmap. Which statement is correct after the last line?

```rust
use std::collections::HashMap;

fn main() {
    let plane = String::from("Boeing");
    let passenger_count = 160;
    let travel_options = HashMap::from([(plane, passenger_count)]);
}

```

`plane` is a valid name while `passenger_count` is an invalid name.
! `plane` is an invalid name while `passenger_count` is a valid name.
Both `plane` and `passenger_count` are invalid names.
Both `plane` and `passenger_count` are valid names.

---

What does the `remove` method return?

! An `Option` containing the value for a given key
An `Option` containing a reference to a value for a given key
An `Option` containing the key
A `Result` containing the value for a given key

---

Which `HashMap` declaration is more versatile?

```rust
use std::collections::HashMap;

fn main() {
    let pixar_movie_characters: HashMap<&String, &String> = HashMap::new();
}

```

OR

```rust
use std::collections::HashMap;

fn main() {
    let pixar_movie_characters: HashMap<&str, &str> = HashMap::new();
}

```

First
! Second

---

What does the `insert` method do?

! Insert a key-value pair to the hash map, replacing an old pair if the given key exists.

Insert a key-value pair to the hash map. Will not replace an old pair if the given key pairs.

Insert a key-value pair to the hash map only if the given key already exists in the hash map.

The `HashMap` does not support the `insert` method.
