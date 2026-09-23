use super::Task;

pub fn add(args: Vec<String>, mut tasks: &Vec<Task>) {
    if args.len() > 2 {
        panic!("Error: extra arguments for add command");
    }

    let mut id: u32 = 0;
    match args.get(1) {
        Some(&arg) => {
            for (i, &task) in tasks.iter().enumerate() {
                id = i.try_into().unwrap();
            }
            tasks.push(Task::new(id, arg));
        }
        None => panic!("You need to add a description for the task"),
    }

    println!("Task added succesfully (ID: {id})");
}

pub fn synopsis() {
    println!("\n\ntask_tracker (command) [id] [text]");
    println!("Commands: <add> <update> <delete> <mark*> <list>");
    println!("mark: <mark-done> <mark-in-progress>");
    println!("list: list done");
    println!("list: list todo");
    println!("list: list in-progress");
}
