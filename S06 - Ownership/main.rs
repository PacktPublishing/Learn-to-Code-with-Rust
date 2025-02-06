fn main() {
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert} {is_event}");

    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi} {dinner}");

    let sushi = String::from("Salmon");
    let dinner = sushi;
    // println!("{dinner} {sushi}");

    let fish = eat_meal(dinner);
    println!("{fish}");
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
