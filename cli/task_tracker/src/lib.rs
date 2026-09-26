use serde::{Serialize, Deserialize};

pub mod utils;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: u32,
    pub description: String,
    pub status: Status,
    created_at: [u8; 3],
    updated_at: [u8; 3],
}

impl Task {
    /*need to add created and updated at*/
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            status: Status::Todo,
            created_at: [0; 3],
            updated_at: [0; 3],
        }
    }

    pub fn display(&self) {
        // TODO must display all other attributes
        println!("{}:\n {}\n", self.id, self.description)
    }

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}
