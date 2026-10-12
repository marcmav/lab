use std::env;
use serde_json::Value;

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
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()?;
    let response = match response.error_for_status() {
        Ok(updated) => updated,
        Err(e) => {
            println!("An Error occured: {}", e);
            return Ok(());
        },
    };
    let events: serde_json::Value = response.json()?;
    println!("Output:");
    for i in 0..10 {
        if events[i]["type"].as_str() == Some("PushEvent") {
            println!("Pushed 3 commits to {}", events[i]["repo"]["name"]);
        }
    }


    Ok(())
}
