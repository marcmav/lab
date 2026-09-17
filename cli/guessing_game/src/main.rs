use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("\nWelcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have a number of chances to guess the correct number.\n");
    println!("Please select the difficulty level:");
    println!("1. Easy (10 chances)");
    println!("2. Medium (5 chances)");
    println!("3. Hard (3 chances)");
    println!("4. Impossible (1 chance)");
    println!("5. Quit\n");
    print!("Enter you choice: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("An error occured while trying to define the difficulty level");
    let chances = match input.trim() {
        "1" | "Easy" => {
            println!("\nGreat! You have selected the Easy difficulty level.");
            10 },
        "2" | "Medium" => {
            println!("\nGreat! You have selected the Medium difficulty level.");
            5 },
        "3" | "Hard" => {
            println!("\nGreat! You have selected the Hard difficulty level.");
            3 },
        "4" | "Impossible" => {
            println!("\nGreat! You have selected the Impossible difficulty level.");
            1 },
        "5" | "Quit" => {
            println!("\nBye bye");
            return; },
        _ => { panic!("Wrong input"); },
    };
    println!("Let's start the game!\n");
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    for i in 1..=chances {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("An error occured while getting the guess");
        if let "Quit" | "Q" | "q" = input.trim() {
            println!("Bye bye");
            return ;
        }
        let guess: u32 = input.trim().parse().expect("Not a valid number");
        if guess == random_number {
            println!("Congratulations! You guessed the correct number in {} attempts", i);
            return;
        } else if guess > random_number {
            println!("Incorrect! The number is less than {}\n", guess);
        } else if guess < random_number {
            println!("Incorrect! The number is greater than {}\n", guess);
        }
    }
    println!("You ran out of attempts, you lost!");
    println!("The random number was {random_number}");
}
