use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    println!("{:?}", sauces_to_meals.remove("Mayonnaise").unwrap());

    let mustard_meals = sauces_to_meals.get("Mustard");
    match mustard_meals {
        Some(meals) => println!("The meals were {meals:?}"),
        None => println!("There were no meals for that sauce! Oh no"),
    }

    sauces_to_meals
        .entry("Ketchup")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{sauces_to_meals:?}");
}
