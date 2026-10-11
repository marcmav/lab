use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args()
        .collect();
 
    let username: String = match args.get(1) {
        Some(user) => user.to_string(),
        None => return Ok(()),
    };

    if let Some(_) = args.get(2) {
        panic!("Error: Too many arguments");
    }

    let url: String = format!("https://api.github.com/users/{username}/events");
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "marcmav")
        .send()?;
    println!("Status {}", response.status());

    Ok(())
}
