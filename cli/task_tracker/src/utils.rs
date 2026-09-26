use super::Task;

pub fn add(args: Vec<String>, tasks: &mut Vec<Task>) {
    if args.len() > 2 {
        panic!("Error: extra arguments for add command");
    }

    let mut id: u32 = 0;
    match args.get(1) {
        Some(arg) => {
            for (i, _) in tasks.iter().enumerate() {
                id = i.try_into().expect("Error: reached maximum number of IDs");
            }
            tasks.push(Task::new(id, arg.to_string()));
        }
        None => panic!("Error: You need to add a description for the task"),
    }

    println!("Task added succesfully (ID: {id})");
}

pub fn list(args: Vec<String>, tasks: &Vec<Task>) {
    match args.get(1) {
        Some(arg) => (), // TODO deal status
        None => {
            for task in tasks.iter() {
                task.display();
            }
        },
    }
}

pub fn update(args: Vec<String>, tasks: &mut Vec<Task>) {
    if args.len() > 3 {
        panic!("Error: extra arguments for update command");
    }
    let id: u32;
    match args.get(1) {
        Some(arg) => match arg.parse() {
            Ok(n) => id = n,
            Err(_) => panic!("Error: the ID must be a number"),
        },
        None => panic!("Error: You need to add the ID of the task"),
    }
    match args.get(2) {
        Some(arg) => {
            let mut updated: bool = false;
            for task in tasks.iter_mut() {
                if task.id == id {
                    task.update_description(arg.to_string());
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

pub fn synopsis() {
    println!("\n\ntask_tracker (command) [id] [text]");
    println!("Commands: <add> <update> <delete> <mark*> <list>");
    println!("mark: <mark-done> <mark-in-progress>");
    println!("list: list done");
    println!("list: list todo");
    println!("list: list in-progress");
}
