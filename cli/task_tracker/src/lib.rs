use serde::{Serialize, Deserialize};

pub mod utils;

// TODO add display trait on Status
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: u32,
    pub status: Status,
    pub description: String,
    created_at: [u8; 3],
    updated_at: [u8; 3],
}

impl Task {
    /*need to add created and updated at*/
    pub fn new(id: u32, description: String, status: Status) -> Self {
        Self {
            id,
            description,
            status,
            created_at: [0; 3],
            updated_at: [0; 3],
        }
    }

    pub fn display(&self) {
        // TODO must display created and updated at
        println!("id {}", self.id);
        println!("status: {:?}", self.status); // TODO must display with no Debug
        println!("description: {}\n", self.description);
    }

    pub fn filter_display(tasks: &Vec<Self>, status: Status) {
        for task in tasks.iter() {
            if task.status == status {
                task.display();
            }
        }
    }

    pub fn get_id(args: &Vec<String>) -> u32 {
        match args.get(1) {
            Some(arg) => match arg.parse::<u32>() {
                Ok(n) => return n,
                Err(_) => panic!("Error: The ID must be a number"),
            },
            None => panic!("Error: You need to add the ID of the task"),
        }
    }
}
