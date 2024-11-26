```rust
/*
Define a Flight struct with the following fields:
  - an 'origin' field (String)
  - a 'destination' field (String)
  - a 'price' field (f64)
  - a 'passengers' field (u32)

Derive a Debug trait implementation for the Flight struct

Define a 'new' constructor function that returns a new
instance of a Flight

Define a 'change_destination` method that accepts a new destination
and overwrites the value of the 'destination' field

Define a 'increase_price' method that raises the value
of the 'price' by 10% (multiply the 'price' field by 1.10).
Make sure to save the new 'price' field value.

Define a 'itinerary' method that prints out both the
'origin' and 'destination' fields in the following format
(origin -> destination)

Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.

Use struct update syntax to copy the 'price' and 'passengers'
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/

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
        self.price *= 1.1;
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

    my_flight.change_destination(String::from("San Francisco"));
    my_flight.increase_price();
    my_flight.itinerary();
    println!("{my_flight:?}");

    let another_flight = Flight {
        origin: String::from("Paris"),
        destination: String::from("London"),
        ..my_flight
    };
    println!("{another_flight:#?}");
}

```
