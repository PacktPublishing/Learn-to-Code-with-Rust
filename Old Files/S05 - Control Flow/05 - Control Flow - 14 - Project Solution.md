```rust
/*
Define a `color_to_number` function that accepts a 'color'
parameter of a string (&str). Use if, else if, and else
statements to return a corresponding numeric value.
If the color is "red", return 1.
If the color is "green", return 2.
If the color is "blue", return 2.
If the color is any other string, return 0.

Refactor the function above to use the match statement
instead of if and else if.

Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
For example:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
The factorial of 4 is 4 * 3 * 2 * 1 = 24
Implement two solutions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_2(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial(number - 1)
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("purple"));

    println!("{}", color_to_number_2("red"));
    println!("{}", color_to_number_2("green"));
    println!("{}", color_to_number_2("purple"));

    println!("{}", factorial(5));
}
```
