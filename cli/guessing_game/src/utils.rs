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
    match buf.trim() {
        "Hint" | "hint" | "h" => return 101,
        other => match other.parse::<u32>() {
            Ok(i) => { if i <= 100 { return i; }
                else { panic!("Enter a number between 0 and 100"); }
            },
            _ => panic!("Wrong input"),
        }
    }
}

#[allow(unused)]
pub fn process_game(chances: u32, random_number: u32, input: &mut String, score: &mut Score) {
    let hint: String = match random_number {
        even if even % 2 == 0 => String::from("\nThe random number is an even number"),
        _ => String::from("\nThe random number is an odd number"),
    };
    let mut i = 1;
    while i <= chances {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        let guess = get_user_input(input);
        if guess == 101 {
            println!("{hint}");
            continue;
        }
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
        i += 1;
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
