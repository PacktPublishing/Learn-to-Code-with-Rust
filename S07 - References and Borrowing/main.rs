fn main() {
    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push('.');
    show_itinerary(&trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}

fn visit_boston(trip: &mut String) {
    trip.push_str("Boston");
}

fn show_itinerary(trip: &String) {
    println!("{trip}");
}
