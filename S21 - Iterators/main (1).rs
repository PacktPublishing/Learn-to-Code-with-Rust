use colored::Colorize;

use std::io::{self, Write};

fn main() {
    let word = "trait";
    let input = io::stdin();

    for _ in 1..=6 {
        let mut user_input = String::new();

        input
            .read_line(&mut user_input)
            .expect("Failed to provide input");

        for (word_character, user_character) in word.chars().zip(user_input.trim().chars().take(5))
        {
            if word_character == user_character {
                print!("{}|", format!(" {user_character} ").on_green())
            } else if word.contains(user_character) {
                print!("{}|", format!(" {user_character} ").on_yellow())
            } else {
                print!("{}|", format!(" {user_character} ").on_black())
            }

            io::stdout().flush().unwrap();
        }

        println!();

        if word == user_input.trim() {
            println!("You got it! The word is {word}");
            break;
        }
    }
}
