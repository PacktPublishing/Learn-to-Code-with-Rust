```rust
/*
Declare an i16 integer assigned to any value.
Use the underscore character to add a visual
separator between the numbers.

Cast the i16 to an i8 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let distance: i16 = 1_000;
    let miles = distance as i8;

    let height = 150.34546;
    println!("{height:.3}");

    let with_milk = true;
    let with_sugar = true;
    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let distances = [distance, 2_000, 3_000, 5_000];
    println!("{distances:?}");

    let combo = (distance, height, is_my_type_of_coffee, distances);
    println!("{combo:?}");
}

```
