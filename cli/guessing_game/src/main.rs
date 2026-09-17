mod greet;

use std::io;
use rand::Rng;

fn main() {
    greet::greet_user();
    let difficulty_level = greet::select_difficulty();
    let mut attempt = match difficulty_level.trim() {
        "1" | "easy" => {
            println!("You chose easy difficulty");
            20 },
        "2" | "medium" => {
            println!("You chose medium difficulty");
            10 },
        "3" | "hard" => {
            println!("You chose hard difficulty");
            5 },
        "4" | "quit" => {
            println!("Bye bye");
            return; },
        _ => 0,
    };
    println!("You have {} attempt/s to get it right", attempt);
    println!("Choose a number: ");
    let mut guess: u32;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("An error occured while getting the guess");
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    while attempt > 0 {
        guess = input.trim().parse().expect("Not a valid number");
        if guess == random_number {
            println!("You guessed it right, congratulations!");
            return;
        }
        attempt -= 1;
    }
    println!("You ran out of attempts, you lost!");
}
