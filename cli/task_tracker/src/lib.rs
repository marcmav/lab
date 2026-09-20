pub mod utils;

enum Status {
    todo,
    in_progress,
    done,
}

struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: (u8; 3),
    updated_at: (u8, 3),
}
