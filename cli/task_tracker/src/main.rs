#[warn(dead_code)] //cant go in production;

use task_tracker::*;
use std::env;

fn main() {
    let args: Vec<String> =env::args().skip(1).collect();
    if args.len() == 0 {
        init_task_tracker();
    }
}
