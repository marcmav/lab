use serde::{Serialize, Deserialize};

pub mod utils;

#[derive(Serialize, Deserialize)]
enum Status {
    todo,
    in_progress,
    done,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: (u8; 3),
    updated_at: (u8, 3),
}
