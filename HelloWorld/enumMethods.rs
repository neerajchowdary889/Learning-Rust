#![allow(dead_code)]
use std::io;

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thrusday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => false,
            _ => true,
        }
    }
}

fn main() {
    println!("Type your Input:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let day = match input.trim().to_lowercase().as_str() {
        "monday" => Day::Monday,
        "tuesday" => Day::Tuesday,
        "wednesday" => Day::Wednesday,
        "thursday" => Day::Thrusday,
        "friday" => Day::Friday,
        "saturday" => Day::Saturday,
        "sunday" => Day::Sunday,
        _ => panic!("Invalid input"),
    };

    println!("Is {} a weekday? --> {}", input.trim(), day.is_weekday());
}

