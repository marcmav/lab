mod score;
mod utils;

use rand::Rng;
use score::*;
use utils::*;

fn main() {
    greet_user();
    let mut score = Score::new();
    let mut input = String::new();
    let mut repeat = true;
    while repeat {
        let chances = get_chance();
        println!("Let's start the game!\n");
        let random_number: u32 = rand::thread_rng().gen_range(1..=100);
        process_game(chances, random_number, &mut input, &mut score);
        repeat = retry();
    }
}
