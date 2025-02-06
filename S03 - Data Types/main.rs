fn main() {
    let distance: i32 = 1_337;
    let miles = distance as i16;

    let height = 150.34546;
    println!("{height:.3}");

    let with_milk = true;
    let with_sugar = true;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let distances: [i8; 4] = [13, 23, 75, 100];
    println!("{:#?}", distances);

    let combo = (miles, height, is_my_type_of_coffee, distances);
    println!("{combo:#?}");
}
