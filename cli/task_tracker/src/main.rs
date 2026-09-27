use std::env;
use std::io::{self};
use std::fs::{OpenOptions};
use task_tracker::*;
use utils::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let f = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("TODO")?;

    let mut tasks: Vec<Task> = Vec::new();

    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "add" => add(args, f),
            "update" => update(args, &mut tasks),
            "delete" => delete(args, &mut tasks),
            "mark-in-progress" | "mark-done" => mark(args, &mut tasks),
            "list" => list(args, f),
            _ => synopsis(),
        },
        None => synopsis(),
    }

    Ok(())
}
