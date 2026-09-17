use std::io::{self, Write};

pub fn greet_user() {
    println!("\nWelcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have a number of chances to guess the correct number.\n");
}

pub fn get_chance() -> u32 {
    println!("Please select the difficulty level:");
    println!("1. Easy (10 chances)");
    println!("2. Medium (5 chances)");
    println!("3. Hard (3 chances)");
    print!("Enter you choice: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("An error occured while trying to define the difficulty level");
    match input.trim() {
        "1" | "Easy" => {
            println!("\nGreat! You have selected the Easy difficulty level.");
            return 10; },
        "2" | "Medium" => {
            println!("\nGreat! You have selected the Medium difficulty level.");
            return 5; },
        "3" | "Hard" => {
            println!("\nGreat! You have selected the Hard difficulty level.");
            return 3; },
        _ => { panic!("Wrong input"); },
    };
}
