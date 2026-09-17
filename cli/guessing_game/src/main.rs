mod utils;
mod score;

use utils::*;
use score::*;
use std::io::{self, Write};
use rand::Rng;

fn main() {
    greet_user();
    let chances = get_chance();
    println!("Let's start the game!\n");
    let mut score = Score::new();
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    for i in 1..=chances {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("An error occured while getting the guess");
        let guess: u32 = input.trim().parse().expect("Not a valid number");
        if guess == random_number {
            println!("Congratulations! You guessed the correct number in {} attempts\n", i);
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
}
