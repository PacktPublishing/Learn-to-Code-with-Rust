```rust
/*
Use the 'use' keyword to bring the HashMap type 
into the current's file's namespace

Declare a `sauces_to_meals` HashMap. The keys will be 
string slices and the values will be a vector of string 
slices. Use the from function to populate hash with 2 
key-value pairs:

Key: "Ketchup"
Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]

Key: "Mayonnaise"
Value: Vector of ["Sandwiches", "Burger", "Coleslaw"]

Use the `insert` method to add a new key-value pair to 
the HashMap.

Key: "Mustard"
Value: Vector of ["Hot dog", "Burgers", "Pretzels"]

Use the `remove` method to remove the key-value pair 
where "Mayonnaise" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `get` method to retrieve the key-value pair
where "Mustard" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Note: There is a cool method on an Option called 'cloned'
that converts an Option<&T> to an Option<T> by cloning 
the contents. Our HashMap values are vectors, which do 
not implement the Copy trait, so the Option does not 
support the copied method.

Use the `entry` and `or_insert` methods to add a new 
key-value pair.

Key: "Soy Sauce"
Value: Vector of ["Sushi", "Dumplings"]

Finally, print out the final sauces_to_meals HashMap.

The final result should be:
{
  "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
  "Soy Sauce": ["Sushi", "Dumplings"],
  "Mustard": ["Hot dog", "Burgers", "Pretzels"]
}
*/

use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burger", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    let possible_meal = sauces_to_meals.remove("Mayonnaise");
    println!("{:?}", possible_meal.unwrap());

    let mustard_meals = sauces_to_meals.get("Mustard").cloned();
    println!("{:?}", mustard_meals.unwrap());

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);
    println!("{:?}", sauces_to_meals);
}

```
