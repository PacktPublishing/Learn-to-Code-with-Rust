```rust
/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? Try printing out the value
of both variables to find out.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? Try printing out the value
of both variables to find out.

Repeat the previous example but use a heap String instead.
Will Rust move ownership?
Why is the result different from the previous operation?

The clear method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a meal parameter.
In the main function, invoke eat_meal and pass in your `sushi`
String. What happens when eat_meal runs?

Say we want to keep the "Salmon" String after eat_meal runs.
How can we continue to use the data in the main function?
*/

fn main() {
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert} + {is_event}");

    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi} {dinner}");

    let sushi = String::from("Salmon");
    let dinner = sushi;
    // println!("{sushi} {dinner}");

    let fish = eat_meal(dinner);
    println!("Meal is finished: {fish}");
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
```
