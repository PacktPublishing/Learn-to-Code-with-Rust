use std::io;

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect::<Vec<&str>>()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();
    println!("What is your first name?");
    input
        .read_line(&mut first_name)
        .expect("Failed to collect first name");
    println!("What is your last name?");
    input
        .read_line(&mut last_name)
        .expect("Failed to collect last name");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    let mut amount = String::from("40");
    make_money(&mut amount);
    println!("{amount}");

    let banana = trim_and_capitalize("       banana      ");
    println!("{banana}");

    let collection = elements("Gold!Silver!Platinum");
    println!("{collection:?}");

    let full_name = get_identity();
    println!("{full_name}");
}
