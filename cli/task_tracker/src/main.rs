use std::env;
use std::io::{self, Write, Read};
use std::fs::{OpenOptions};
use task_tracker::*;
use utils::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("TODO.json")?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let mut tasks: Vec<Task>;
    if content.is_empty() {
        tasks = Vec::new();
    } else {
        tasks = serde_json::from_str::<TaskList>(&content)?.tasks;
    }

    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "add" => add(args, &mut tasks),
            "update" => update(args, &mut tasks),
            "delete" => delete(args, &mut tasks),
            "mark-in-progress" | "mark-done" => mark(args, &mut tasks),
            "list" => list(args, &mut tasks),
            _ => panic!("Error: No input was found"),
        },
        None => panic!("Error: No input was found"),
    }

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("TODO.json")?;

    let tasks = TaskList { tasks: tasks, };
    f.write(serde_json::to_string_pretty(&tasks)?.as_bytes())?;

    Ok(())
}
