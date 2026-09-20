pub fn add(args: Vec<String>) -> Result<Value>{
    if args.len() != 2 {
        return io::Error;
    }

    println!("Task added succesfully (ID: {})", id); //must define id
}

pub fn synopsis() {
    println!("\n\ntask_tracker (command) [id] [text]");
    println!("Commands: <add> <update> <delete> <mark*> <list>");
    println!("mark: <mark-done> <mark-in-progress>");
    println!("list: list done");
    println!("list: list todo");
    println!("list: list in-progress");
}
