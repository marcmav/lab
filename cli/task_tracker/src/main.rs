use std::env;
use task_tracker::*;
use utils::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    //must also pass the fd that will hold the tasks
    //i will try saving the stacks on a vector
    let mut tasks: Vec<Task> = Vec::new();
    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "add" => add(args, &mut tasks),
            "update" => update(args, &mut tasks),
            "delete" => delete(args, &mut tasks),
            "mark-in-progress" | "mark-done" => mark(args, &mut tasks),
            "list" => list(args, &tasks),
            _ => synopsis(),
        },
        None => synopsis(),
    }
}
