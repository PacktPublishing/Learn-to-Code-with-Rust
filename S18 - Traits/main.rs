use std::fmt::{Debug, Display, Formatter};

trait Drinkable {
    fn consume(&mut self);

    fn get_data(&self) -> String;

    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("COFFEE")
            .field("Kind", &self.kind)
            .field("Milk", &self.milk)
            .field("Ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            percentage: self.percentage,
            flavor: self.flavor.clone(),
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    let mut latte = Coffee::new("Latte", Milk::Oat, 32);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new(String::from("Cappuccino"), Milk::Oat, 16);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(300, 2.99, String::from("Cherry"));
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("{}", pepsi == coke);
    coke.consume();
    println!("{:?}", coke);
}
