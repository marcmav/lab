mod utils;

use utils::*;
use std::io;
use rand::Rng;

fn main() {
    greet_user();
    let mut attempt = match select_difficulty().trim() {
        "1" | "easy" => {
            println!("You chose easy difficulty");
            20 },
        "2" | "medium" => {
            println!("You chose medium difficulty");
            10 },
        "3" | "hard" => {
            println!("You chose hard difficulty");
            5 },
        "4" | "impossible" => {
            println!("You chose impossible difficulty");
            1 },
        "5" | "quit" => {
            println!("Bye bye");
            return; },
        _ => 0,
    };
    println!("You have {} attempt/s to get it right", attempt);
    println!("Choose a number (between 1 and 100): ");
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    while attempt > 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("An error occured while getting the guess");
        let guess: u32 = input.trim().parse().expect("Not a valid number");
        if guess == random_number {
            println!("You guessed it right, congratulations!");
            return;
        } else if guess > random_number {
            println!("You guessed to high");
        } else if guess < random_number {
            println!("You guessed to low");
        }
        attempt -= 1;
    }
    println!("You ran out of attempts, you lost!");
}
