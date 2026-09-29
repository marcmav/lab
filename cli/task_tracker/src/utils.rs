use super::{Task, Status};

// TODO i can sort by id so it always get organized or better just swap by its index + 1
pub fn add(args: Vec<String>, tasks: &mut Vec<Task>) {
    if args.len() > 2 {
        panic!("Error: Extra arguments for add command");
    }

    match args.get(1) {
        Some(arg) => {
            let task = Task::new(0, arg.to_string(), Status::Todo);
            tasks.push(task);
        }
        None => panic!("Error: You need to add a description for the task"),
    }

    Task::assign_id(tasks);

    println!("Task added succesfully (ID: {})", tasks.last().unwrap().id);
}

/*
pub fn update(args: Vec<String>, tasks: &mut Vec<Task>) {
    if args.len() > 3 {
        panic!("Error: Extra arguments for update command");
    }

    let id: u32 = Task::get_id(&args);

    match args.get(2) {
        Some(arg) => {
            let mut updated: bool = false;
            for task in tasks.iter_mut() {
                if task.id == id {
                    task.description = arg.to_string();
                    updated = true;
                    break;
                }
            }
            if !updated {
                panic!("Error: Unknown ID");
            }
        },
        None => panic!("Error: You need to add a new description of the task"),
    }
}

pub fn delete(args: Vec<String>, tasks: &mut Vec<Task>) {
    if args.len() > 2 {
        panic!("Error: Extra arguments for delete command");
    }

    let mut idx: usize = 0;
    match args.get(1) {
        Some(arg) => match arg.parse::<u32>() {
            Ok(n) => {
                let mut indexed: bool = false;
                for (i, task) in tasks.iter().enumerate() {
                    if task.id == n {
                        idx = i;
                        indexed = true;
                        break;
                    }
                }
                if !indexed {
                    panic!("Error: Unknown iD")
                }
            },
            Err(_) => panic!("Error: Error: The ID must be a number"),
        }
        None => panic!("Error: You need to a add the ID of the task to delete"),
    }
    tasks.remove(idx);
}

pub fn mark(args: Vec<String>, tasks: &mut Vec<Task>) {
    if args.len() > 2 {
        panic!("Error: Extra arguments for mark_in_progress command");
    }

    let id: u32 = Task::get_id(&args);

    let mut updated: bool = false;
    for task in tasks.iter_mut() {
        if task.id == id {
            if args[0].as_str() == "mark-in-progress" {
                task.status = Status::InProgress;
            } else {
                task.status = Status::Done;
            }
            updated = true;
            break;
        }
    }
    if !updated {
        panic!("Error: Unknown ID");
    }
}

pub fn list(args: Vec<String>, f: File) {
    let reader = BufReader::new(&f);

    match args.get(1) {
        Some(arg) => {
            match arg.as_str() {
                "todo" => Task::list(reader, Status::Todo),
                "in-progress" => Task::list(reader, Status::InProgress),
                "done" => Task::list(reader, Status::Done),
                _ => panic!("Error: Invalid task status\nValids: <done> <todo> <in-progress>"),
            }
        }
        None => Task::list(reader, Status::Todo),
    }
}
*/

pub fn synopsis() {
    println!("\n\ntask_tracker (command) [id] [text]");
    println!("Commands: <add> <update> <delete> <mark*> <list>");
    println!("mark: <mark-done> <mark-in-progress>");
    println!("list: list done");
    println!("list: list todo");
    println!("list: list in-progress");
}
