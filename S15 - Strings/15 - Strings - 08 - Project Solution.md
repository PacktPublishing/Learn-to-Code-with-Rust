```rust
/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.

Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.

Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the ! symbol and return a vector of the string
slices. Invoke the function in `main`.

Example:
text("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]

Define a `full_name` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the expect
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `full_name` function in `main`, and output the
returned String.

Example:
fn main() {
  let name = full_name();
   println!("{name}");
}
*/

use std::io;

fn main() {
    let name = full_name();
    println!("{name}");
}

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect()
}

fn full_name() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let stdin = io::stdin();
    println!("What is your first name?");
    stdin
        .read_line(&mut first_name)
        .expect("Failed to collect first name");
    println!("What is your last name?");
    stdin
        .read_line(&mut last_name)
        .expect("Failed to collect last name");
    format!("{} {}", first_name.trim(), last_name.trim())
}

```
