use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args()
        .collect();
 
    let username: String = match args.get(1) {
        Some(user) => user.to_string(),
        None => return Ok(()),
    };

    if let Some(_) = args.get(2) {
        panic!("Error: Too many arguments");
    }

    Ok(())
}
