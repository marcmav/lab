use crate::score::*;
use std::io::{self, Write};

pub fn greet_user() {
    println!("\nWelcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have a number of chances to guess the correct number.");
}

pub fn get_chance() -> u32 {
    println!("\nPlease select the difficulty level:");
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
            return 10;
        }
        "2" | "Medium" => {
            println!("\nGreat! You have selected the Medium difficulty level.");
            return 5;
        }
        "3" | "Hard" => {
            println!("\nGreat! You have selected the Hard difficulty level.");
            return 3;
        }
        _ => {
            panic!("Wrong input");
        }
    };
}

pub fn get_user_input(buf: &mut String) -> u32 {
    buf.clear();
    io::stdin()
        .read_line(buf)
        .expect("An error occured while getting the guess");
    buf.trim().parse::<u32>().expect("Not a valid number")
}

pub fn process_game(chances: u32, random_number: u32, input: &mut String, score: &mut Score) {
    for i in 1..=chances {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        let guess = get_user_input(input);
        if guess == random_number {
            println!(
                "Congratulations! You guessed the correct number in {} attempts\n",
                i
            );
            score.update_score(chances, i);
            score.print_scores();
            return;
        } else if guess > random_number {
            println!("Incorrect! The number is less than {}\n", guess);
        } else if guess < random_number {
            println!("Incorrect! The number is greater than {}\n", guess);
        }
    }
    println!("You Lost! the random number was {random_number}");
    score.print_scores();
}

pub fn retry() -> bool {
    let mut input = String::new();
    println!("\nWould you like to try again?");
    println!("1. Yes");
    println!("2. No");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("An error occured while getting response");
    match input.trim() {
        "Yes" | "yes" | "1" => true,
        "No" | "no" | "2" => false,
        _ => panic!("Wrong input"),
    }
}
