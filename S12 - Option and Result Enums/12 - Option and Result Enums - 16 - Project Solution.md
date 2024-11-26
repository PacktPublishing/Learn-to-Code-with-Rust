```rust
/*
Define a Food struct with a single `name` field
set to a String. Derive a Debug implementation.

Define a Restaurant struct with a `reservations` field
set to a u32 and a `has_mice_infestation` field set to
a bool. Derive a Debug implementation.

Define a `book_reservation` method on the Restaurant.
If should return an Option containing a boolean.
If the restaurant has a mice infestation, return the
None variant.
If the restaurant has less than 12 reservations, return
the Some variant holding a true value.
Otherwise, return the Some variant holding a false value.

Define a `cook_meal` method on the Restaurant.
It should accept a `name` String representing the
name of the meal. It should return a Result type
where the Ok variant holds a Food struct and the Err
variant holds a String.
If the restaurant has a mice infestation, return the
Err variant containing a String of "Sorry, we have a
mice problem".
Otherwise, return the Ok variant containing a Food
struct. The Food struct's `name` field value should
come from the `name` function parameter.
*/

#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn book_reservation(&self) -> Option<bool> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn cook_meal(&self, name: String) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err(String::from("Sorry, we have a mice problem"))
        } else {
            Ok(Food { name })
        }
    }
}

fn main() {
    let marios = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    let reservation = marios.book_reservation();
    println!("{:?}", reservation);

    let meal = marios.cook_meal(String::from("Lasagna"));
    println!("{:?}", meal);

    let angelos = Restaurant {
        reservations: 12,
        has_mice_infestation: false,
    };

    let reservation = angelos.book_reservation();
    println!("{:?}", reservation);

    let meal = angelos.cook_meal(String::from("Lasagna"));
    println!("{:?}", meal);
}

```
