use std::env;
use task_tracker::*;
use utils::*;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    match args.get(0) {
        Some(arg) => match arg.to_str() {
            "add" => add_task(),
            "update" => update_task(),
            "delete" => delete_task(),
            "list" => list_task(),
            "mark-in-progress" => mark_in_progress(),
            "mark-done" => mark_done(),
            _ => return, //throw a proper error
        },
        None => synopsis(), //throw a proper error
    }
}
