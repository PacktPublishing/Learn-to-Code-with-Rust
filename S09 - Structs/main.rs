#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut my_flight = Flight::new(
        String::from("New York"),
        String::from("Los Angeles"),
        299.99,
        150,
    );

    println!("{:?}", my_flight);
    my_flight.change_destination(String::from("San Francisco"));
    my_flight.increase_price();
    my_flight.itinerary();
    println!("{:?}", my_flight);

    let another_flight = Flight {
        origin: String::from("Paris"),
        destination: String::from("Rome"),
        ..my_flight
    };

    println!("{:#?}", another_flight);
}
