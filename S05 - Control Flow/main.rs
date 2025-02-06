fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_iterative(number: i32) -> i32 {
    let mut product = 1; // product = 1
    let mut count = number; // count = 5

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn factorial_recursive(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial_recursive(number - 1)
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number("purple"));

    println!("{}", factorial_iterative(4));
    println!("{}", factorial_recursive(4));
}
