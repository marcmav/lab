use serde::{Serialize, Deserialize};

pub mod utils;

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
    created_at: [u8; 3],
    updated_at: [u8; 3],
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
            created_at: [0; 3],
            updated_at: [0; 3],
        }
    }

    pub fn assign_id(tasks: &mut Vec<Self>) {
        for (i, task) in tasks.iter_mut().enumerate() {
            task.id = i as u32 + 1;
        }
    }

    /*
    pub fn display(&self) {
        // TODO must display created and updated at
        println!("id {}", self.id);
        println!("status: {:?}", self.status); // TODO must display with no Debug
        println!("description: {}\n", self.description);
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

    pub fn extract_id(line: String) -> u32 {
        line
            .chars()
            .skip(4)
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }
 
    // TODO should return &str instead (must learn lifetime)
    pub fn status_to_string(status: Status) -> String {
        let status = match status {
            Status::Todo => "Todo",
            Status::InProgress => "InProgress",
            Status::Done => "Done",
        };
        status.to_string()
    }

    pub fn list(f_reader: BufReader<&File>, status: Status) {
        let f_reader: Vec<String> = f_reader.lines()
            .map(|line| line.expect("Error: Can't read line"))
            .collect();

        let status = Self::status_to_string(status);

        for lines in f_reader.chunks(4) {
            if lines[1].contains(status.as_str()) {
                for line in lines {
                    println!("{line}");
                }
            }
        }
    }
    */
}
