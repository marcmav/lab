use std::io;

pub fn greet_user() {
    println!("==================================================");
    println!("                  Guessing Game                   ");
    println!("==================================================");
    println!("Guess the correct number and win the prize.");
    println!("                     Rules:");
    println!("--------------------------------------------------");
    println!("1. Choose a number between 1 and 100");
    println!("2. Choose decimal numbers in numerical format");
    println!("3. You only have a set of attempts");
}

pub fn select_difficulty() -> String {
    println!("--------------------------------------------------");
    println!("Select Difficulty Level (The number of attempts allowed)");
    println!("--------------------------------------------------");
    println!("1. easy");
    println!("2. medium");
    println!("3. hard");
    println!("4. impossible");
    println!("5. quit");
    println!("--------------------------------------------------");
    println!("Select (eg.: easy or 1): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("An error occured while trying to define the difficulty level");
    match input.trim() {
        "easy" | "medium" | "hard" | "impossible" | "quit" => (),
        "1" | "2" | "3" | "4" | "5" => (),
        _ => { panic!("Wrong input"); },
    }
    input
}
