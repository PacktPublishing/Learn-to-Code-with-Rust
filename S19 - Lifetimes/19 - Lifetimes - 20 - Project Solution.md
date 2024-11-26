```rust
/*
Define a 'capacity_of' function that accepts a reference
to a vector. The function should be able to accept a vector
storing any data type. Return the capacity of the vector (you
can acquire this with the 'capacity' method). The function
accepts a reference parameter. Does this function need
lifetime annotations? Explain why or why not.

Define a 'first_five' function that accepts two string
slice parameters: 'text' and 'announcement'. The function
should print the value of 'announcement' and return a slice
of the first 5 bytes of 'text'. Does this function need
lifetime annotations? Explain why or why not.

Define a 'clear_whitespace' function that accepts two
string slices. If the first string slice is longer than
the second, remove the whitespace from the first string
and return the result. If the second string slice is longer
tha the first, return the whitespace from the second string
and return the result. Does this functio need lifetime
annotations? Explain why or why not.
*/

fn capacity_of<T>(collection: &Vec<T>) -> usize {
    collection.capacity()
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[0..5]
}

fn clear_whitespace<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first.trim()
    } else {
        second.trim()
    }
}

fn main() {
    let collection = vec![1, 2, 3];
    println!("{}", collection.capacity());

    let five = first_five("Macchiato", "Welcome!");
    println!("{five}");

    let evaluation = clear_whitespace(" morning ", "  breakfast  ");
    println!("{evaluation}");
}

```