use serde::{Serialize, Deserialize};

// TODO add display trait on Status
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    pub status: Status,
    pub description: String,
    /*
    created_at: Date,
    updated_at: Date,
    */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl Task {
    // TODO need to add created and updated at
    pub fn new(id: u32, description: String, status: Status) -> Self {
        Self {
            id,
            description,
            status,
            /*
            created_at: Date (0, 0, 0),
            updated_at: Date (0, 0, 0),
            */
        }
    }

    pub fn assign_id(tasks: &mut Vec<Self>) {
        for (i, task) in tasks.iter_mut().enumerate() {
            task.id = i as u32 + 1;
        }
    }

    pub fn get_id(args: &Vec<String>) -> u32 {
        match args.get(1) {
            Some(arg) => match arg.parse::<u32>() {
                Ok(n) => n,
                Err(_) => panic!("Error: The ID must be a number"),
            },
            None => panic!("Error: You need to add the ID of the task"),
        }
    }

    pub fn display(&self) {
        // TODO must display created and updated at
        println!("id {}", self.id);
        println!("status: {:?}", self.status); // TODO must display with no Debug
        println!("description: {}\n", self.description);
    }

    pub fn list(tasks: &Vec<Task>, status: Status) {
        for task in tasks.iter() {
            if task.status == status {
                task.display()
            }
        }
    }
}

pub mod utils;
