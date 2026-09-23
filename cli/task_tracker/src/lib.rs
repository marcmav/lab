use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Copy)]
struct Task {
    id: u32,
    description: String,
    status: Status,
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
}

pub mod utils;
