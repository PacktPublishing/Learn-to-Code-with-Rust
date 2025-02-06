```rust
/*
Create a `start_trip` function that returns a String of
"The plan is..."

Invoke the `start_trip` function in `main` and save its
return value to a `trip` variable.

We want to pass the String to three separate functions
that will mutate the String without transferring ownership.

Define an `add_philadelphia` function that concatenates
the text "Philadelphia " to the end of the String.

Define an `add_new_york` function that concatenates the
text "and New York " to the end of the String.

Define an `add_boston` function that concatenates the
text "and Boston" to the end of the String.

Define a `show_itinerary` function that will print out
the final version of the String. Find a way to do so
without transferring ownership.

The String's final content should be
"The plan is...Philadelphia and New York and Boston"
*/

fn main() {
    let mut trip = String::from("The plan is... ");
    add_philadelphia(&mut trip);
    add_new_york(&mut trip);
    add_boston(&mut trip);
    show_itinerary(&trip);
}

fn add_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia ")
}

fn add_new_york(trip: &mut String) {
    trip.push_str("and New York ")
}

fn add_boston(trip: &mut String) {
    trip.push_str("and Boston")
}

fn show_itinerary(trip: &String) {
    println!("{trip}");
}

```
