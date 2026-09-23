use std::env;
use task_tracker::*;
use utils::*;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    //must also pass the fd that will hold the tasks
    //i will try saving the stacks on a vector
    let mut tasks: Vec<Task> = Vec::new();
    match args.get(0) {
        Some(arg) => match arg.to_str() {
            "add" => add(&args, &mut tasks),
            /*
            "update" => update_task(),
            "delete" => delete_task(),
            "list" => list_task(),
            "mark-in-progress" => mark_in_progress(),
            "mark-done" => mark_done(),
            */
            _ => synopsis(),
        },
        None => synopsis(),
    }
}
